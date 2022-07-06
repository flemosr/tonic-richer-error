use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;
use tonic::{codegen::Bytes, Code, Status};

mod error_detail;

mod pb {
    include!("./pb/google.rpc.rs");
}

pub use error_detail::{
    BadRequest, DebugInfo, ErrorDetail, ErrorInfo, PreconditionFailure, QuotaFailure, RequestInfo,
    RetryInfo,
};

trait IntoAny {
    fn into_any(self) -> Result<Any, EncodeError>;
}

trait FromAny {
    fn from_any(any: Any) -> Result<Self, DecodeError>
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

        for error_detail in details.into_iter() {
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
                ErrorDetail::ErrorInfo(error_info) => {
                    let any = error_info.into_any()?;
                    conv_details.push(any);
                }
                ErrorDetail::PreconditionFailure(prec_failure) => {
                    let any = prec_failure.into_any()?;
                    conv_details.push(any);
                }
                ErrorDetail::BadRequest(bad_req) => {
                    let any = bad_req.into_any()?;
                    conv_details.push(any);
                }
                ErrorDetail::RequestInfo(req_info) => {
                    let any = req_info.into_any()?;
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

        for any in status.details.into_iter() {
            match any.type_url.as_str() {
                RetryInfo::TYPE_URL => {
                    let retry_info = RetryInfo::from_any(any)?;
                    details.push(retry_info.into());
                }
                DebugInfo::TYPE_URL => {
                    let debug_info = DebugInfo::from_any(any)?;
                    details.push(debug_info.into());
                }
                QuotaFailure::TYPE_URL => {
                    let quota_failure = QuotaFailure::from_any(any)?;
                    details.push(quota_failure.into());
                }
                ErrorInfo::TYPE_URL => {
                    let error_info = ErrorInfo::from_any(any)?;
                    details.push(error_info.into());
                }
                PreconditionFailure::TYPE_URL => {
                    let prec_failure = PreconditionFailure::from_any(any)?;
                    details.push(prec_failure.into());
                }
                BadRequest::TYPE_URL => {
                    let bad_req = BadRequest::from_any(any)?;
                    details.push(bad_req.into());
                }
                RequestInfo::TYPE_URL => {
                    let error_info = RequestInfo::from_any(any)?;
                    details.push(error_info.into());
                }
                _ => {}
            }
        }

        Ok(details)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::Duration;
    use tonic::{Code, Status};

    use super::{
        BadRequest, DebugInfo, ErrorInfo, PreconditionFailure, QuotaFailure, RequestInfo,
        RetryInfo, WithErrorDetails,
    };

    #[test]
    fn gen_status() {
        let mut metadata = HashMap::new();
        metadata.insert("instanceLimitPerRequest", "100");

        let details = vec![
            RetryInfo::with_retry_delay(Duration::from_secs(5)).into(),
            DebugInfo::with_stack(vec!["trace3", "trace2", "trace1"], "details").into(),
            QuotaFailure::with_violation("clientip:<ip address>", "description").into(),
            PreconditionFailure::with_violation(
                "TOS",
                "example.local",
                "Terms of service not accepted",
            )
            .into(),
            ErrorInfo::with_data("SOME_INFO", "mydomain.com", metadata).into(),
            BadRequest::with_violation("field", "description").into(),
            RequestInfo::with_data("request-id", "some-request-data").into(),
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
