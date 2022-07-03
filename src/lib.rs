use prost::{DecodeError, Message};
use prost_types::Any;
use tonic::{codegen::Bytes, Code, Status};

mod pb {
    include!("./pb/google.rpc.rs");
}

mod bad_request;

pub use bad_request::BadRequest;

#[derive(Debug)]
pub enum ErrorDetail {
    // RetryInfo,
    // DebugInfo,
    // QuotaFailure,
    // ErrorInfo,
    // PreconditionFailure,
    BadRequest(BadRequest),
    // RequestInfo,
    // ResourceInfo,
    // Help,
    // LocalizedMessage,
}

pub trait ToAny {
    fn to_any(&self) -> Any;
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
        details: Vec<impl ToAny>,
    ) -> Status;

    fn extract_error_details(&self) -> Result<Vec<ErrorDetail>, DecodeError>;
}

impl WithErrorDetails for Status {
    fn with_error_details(
        code: Code,
        message: impl Into<String>,
        details: Vec<impl ToAny>,
    ) -> Self {
        let message: String = message.into();

        let status = pb::Status {
            code: code as i32,
            message: message.clone(),
            details: details.iter().map(|v| v.to_any()).collect(),
        };

        let mut details_bytes: Vec<u8> = Vec::new();
        details_bytes.reserve(status.encoded_len());
        status.encode(&mut details_bytes).unwrap();

        let details_bytes = Bytes::from(details_bytes);

        Status::with_details(code, message, details_bytes)
    }

    fn extract_error_details(&self) -> Result<Vec<ErrorDetail>, DecodeError> {
        let status = pb::Status::decode(self.details())?;

        let mut details: Vec<ErrorDetail> = Vec::with_capacity(status.details.len());

        for any in status.details.iter() {
            match any.type_url.as_str() {
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

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
