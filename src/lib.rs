use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;
use tonic::{codegen::Bytes, Code, Status};

mod pb {
    include!("./pb/google.rpc.rs");
}

mod retry_info;

pub use retry_info::RetryInfo;

mod debug_info;

pub use debug_info::DebugInfo;

mod quota_failure;

pub use quota_failure::QuotaFailure;

mod bad_request;

pub use bad_request::BadRequest;

#[derive(Debug)]
pub enum ErrorDetail {
    RetryInfo(RetryInfo),
    DebugInfo(DebugInfo),
    QuotaFailure(QuotaFailure),
    // ErrorInfo,
    // PreconditionFailure,
    BadRequest(BadRequest),
    // RequestInfo,
    // ResourceInfo,
    // Help,
    // LocalizedMessage,
}

impl From<RetryInfo> for ErrorDetail {
    fn from(err_detail: RetryInfo) -> Self {
        ErrorDetail::RetryInfo(err_detail)
    }
}

impl From<DebugInfo> for ErrorDetail {
    fn from(err_detail: DebugInfo) -> Self {
        ErrorDetail::DebugInfo(err_detail)
    }
}

impl From<QuotaFailure> for ErrorDetail {
    fn from(err_detail: QuotaFailure) -> Self {
        ErrorDetail::QuotaFailure(err_detail)
    }
}

impl From<BadRequest> for ErrorDetail {
    fn from(err_detail: BadRequest) -> Self {
        ErrorDetail::BadRequest(err_detail)
    }
}

trait IntoAny {
    fn into_any(&self) -> Result<Any, EncodeError>;
}

trait FromAny {
    fn from_any(any: &Any) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

pub trait WithErrorDetails {
    fn with_error_details(
        code: tonic::Code,
        message: impl Into<String>,
        details: Vec<ErrorDetail>,
    ) -> Result<Status, EncodeError>;

    fn extract_error_details(&self) -> Result<Vec<ErrorDetail>, DecodeError>;
}

impl WithErrorDetails for Status {
    fn with_error_details(
        code: Code,
        message: impl Into<String>,
        details: Vec<ErrorDetail>,
    ) -> Result<Self, EncodeError> {
        let message: String = message.into();

        let mut conv_details: Vec<Any> = Vec::with_capacity(details.len());

        for error_detail in details.iter() {
            match error_detail {
                ErrorDetail::RetryInfo(retry_info) => {
                    let any = retry_info.into_any()?;
                    conv_details.push(any);
                }
                ErrorDetail::DebugInfo(debug_info) => {
                    let any = debug_info.into_any()?;
                    conv_details.push(any);
                }
                ErrorDetail::QuotaFailure(quota_failure) => {
                    let any = quota_failure.into_any()?;
                    conv_details.push(any);
                }
                ErrorDetail::BadRequest(bad_req) => {
                    let any = bad_req.into_any()?;
                    conv_details.push(any);
                }
            }
        }

        let status = pb::Status {
            code: code as i32,
            message: message.clone(),
            details: conv_details,
        };

        let mut buf: Vec<u8> = Vec::new();
        buf.reserve(status.encoded_len());
        status.encode(&mut buf)?;

        Ok(Status::with_details(code, message, Bytes::from(buf)))
    }

    fn extract_error_details(&self) -> Result<Vec<ErrorDetail>, DecodeError> {
        let status = pb::Status::decode(self.details())?;

        let mut details: Vec<ErrorDetail> = Vec::with_capacity(status.details.len());

        for any in status.details.iter() {
            match any.type_url.as_str() {
                RetryInfo::TYPE_URL => {
                    let retry_info = RetryInfo::from_any(any)?;
                    details.push(ErrorDetail::RetryInfo(retry_info));
                }
                DebugInfo::TYPE_URL => {
                    let debug_info = DebugInfo::from_any(any)?;
                    details.push(ErrorDetail::DebugInfo(debug_info));
                }
                QuotaFailure::TYPE_URL => {
                    let quota_failure = QuotaFailure::from_any(any)?;
                    details.push(ErrorDetail::QuotaFailure(quota_failure));
                }
                BadRequest::TYPE_URL => {
                    let bad_req = BadRequest::from_any(any)?;
                    details.push(ErrorDetail::BadRequest(bad_req));
                }
                _ => {}
            }
        }

        Ok(details)
    }
}

#[cfg(test)]
mod tests {

    use std::time::Duration;
    use tonic::{Code, Status};

    use super::{BadRequest, DebugInfo, QuotaFailure, RetryInfo, WithErrorDetails};

    #[test]
    fn gen_status() {
        let details = vec![
            RetryInfo::with_retry_delay(Duration::from_secs(5)).into(),
            DebugInfo::with_stack(vec!["trace3", "trace2", "trace1"], "details").into(),
            QuotaFailure::with_violation("clientip:<ip address>", "description").into(),
            BadRequest::with_violation("field", "description").into(),
        ];

        let fmt_details = format!("{:?}", details);

        println!("{fmt_details}\n");

        let status = match Status::with_error_details(
            Code::InvalidArgument,
            "error with bad request details",
            details,
        ) {
            Ok(status) => status,
            Err(err) => panic!("Error generating status: {:?}", err),
        };

        println!("{:?}\n", status);

        let ext_details = match status.extract_error_details() {
            Ok(ext_details) => ext_details,
            Err(err) => panic!("Error extracting details from status: {:?}", err),
        };

        let ext_details = format!("{:?}", ext_details);

        println!("{ext_details}");

        assert!(
            fmt_details.eq(&ext_details),
            "Extracted details differs from original details"
        );
    }
}
