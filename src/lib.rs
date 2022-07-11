/*!
This crate introduces the `WithErrorDetails` trait and implements it in
`tonic::Status`, allowing the implementation of the
[gRPC Richer Error Model](https://www.grpc.io/docs/guides/error/) with
[tonic](https://docs.rs/tonic/latest/tonic/) in a convenient way.

# Usage
The `WithErrorDetails` trait adds associated functions to `tonic::Status` that
can be used on the server side to create a status with error details, that can
then be returned to the gRPC client. Moreover, the trait also adds methods
to `tonic::Status` that can be used by a tonic client to extract error details,
and handle them with ease.

# Getting Started
```toml
[dependencies]
tonic-richer-error = "0.2.0"
```

# Examples
The examples bellow cover a basic use case. A more complete server and client
implementation can be found at [github](https://github.com/flemosr/tonic-richer-error).

## Server Side: Generating `tonic::Status` with an `ErrorDetails` struct
```
use tonic::{Code, Status};
use tonic_richer_error::{ErrorDetails, WithErrorDetails};

// ... inside a gRPC server endpoint method that returns Result<Response<PbRes>, Status>

// Create empty ErrorDetails struct
let mut err_details = ErrorDetails::new();

// Add error details conditionally
if true {
    err_details.add_bad_request_violation(
        "field_a",
        "description of why the field_a is invalid"
    );
}

if true {
    err_details.add_bad_request_violation(
        "field_b",
        "description of why the field_b is invalid",
    );
}

// Check if any error details were set and return error status if so
if err_details.has_bad_request_violations() {

    // Add aditional error details if necessary
    err_details
        .add_help_link("description of link", "https://resource.example.local")
        .set_localized_message("en-US", "message for the user");

    let status = Status::with_error_details(
        Code::InvalidArgument,
        "bad request",
        err_details,
    )
    .unwrap();

    // Here the status would be returned. Omitted to avoid breaking tests
    // return Err(status);
}

// Deal with valid request

// ...

```

## Client Side: Extracting an `ErrorDetails` struct from `tonic::Status`
```
use tonic::{Response, Status};
use tonic_richer_error::{WithErrorDetails};

// ... where req_result is returned by a tonic::Client endpoint method

fn handle_req_result<T>(req_result: Result<Response<T>, Status>) {
    match req_result {
        Ok(_) => {
            // deal with valid response
        },
        Err(status) => {
            let err_details = status.get_error_details().unwrap();
            if let Some(bad_request) = err_details.bad_request {
                // deal with bad_request details
            }
            if let Some(help) = err_details.help {
                // deal with help details
            }
            if let Some(localized_message) = err_details.localized_message {
                // deal with localized_message details
            }
        }
    };
}
```

## Setup different standard error messages
Multiple examples are provided at the [ErrorDetails](struct.ErrorDetails.html)
docs. The [standard error messages](https://github.com/googleapis/googleapis/blob/master/google/rpc/error_details.proto)
provide instructions about how to setup the messages fields correctly.

## Alternative `tonic::Status` associated functions and methods
The [WithErrorDetails](trait.WithErrorDetails.html) docs provide examples with
an alternative way of interacting with `tonic::Status` using vectors of
standard error messages directly (
[::with_error_details_vec](trait.WithErrorDetails.html#tymethod.with_error_details_vec),
[.get_error_details_vec](trait.WithErrorDetails.html#tymethod.get_error_details_vec)
), which can provide more control over the final error details vector if
necessary. Besides that, multiple examples with alternative error detail
extration methods are provided, which can be useful if only one kind of error
detail is being used, for example: [.get_details_bad_request](trait.WithErrorDetails.html#tymethod.get_details_bad_request).


*/

use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;
use tonic::{codegen::Bytes, Code, Status};

mod pb {
    include!("./pb/google.rpc.rs");
}

mod error_detail;
mod error_details;
mod error_details_vec;

pub use error_detail::*;

pub use error_details::ErrorDetails;

pub use error_details_vec::ErrorDetail;

trait IntoAny {
    fn into_any(self) -> Result<Any, EncodeError>;
}

trait FromAny {
    fn from_any(any: Any) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

/// Adds the crate core functionality to `tonic::Status`.
pub trait WithErrorDetails {
    /// Generates a `tonic::Status` with error details obtained from an
    /// `ErrorDetails` struct.
    /// # Examples
    ///
    /// ```
    /// use tonic::{Code, Status};
    /// use tonic_richer_error::{ErrorDetails, WithErrorDetails};
    ///
    /// let status = Status::with_error_details(
    ///     Code::InvalidArgument,
    ///     "bad request",
    ///     ErrorDetails::with_bad_request_violation("field", "description"),
    /// )
    /// .unwrap();
    /// ```
    fn with_error_details(
        code: tonic::Code,
        message: impl Into<String>,
        details: ErrorDetails,
    ) -> Result<Status, EncodeError>;

    /// Generates a `tonic::Status` with error details provided in a vector of
    /// `ErrorDetail` enums.
    /// # Examples
    ///
    /// ```
    /// use tonic::{Code, Status};
    /// use tonic_richer_error::{BadRequest, WithErrorDetails};
    ///
    /// let status = Status::with_error_details_vec(
    ///     Code::InvalidArgument,
    ///     "bad request",
    ///     vec![
    ///         BadRequest::with_violation("field", "description").into(),
    ///     ]
    /// )
    /// .unwrap();
    /// ```
    fn with_error_details_vec(
        code: tonic::Code,
        message: impl Into<String>,
        details: Vec<ErrorDetail>,
    ) -> Result<Status, EncodeError>;

    /// Get an `ErrorDetails` struct from a `tonic::Status`.
    /// # Examples
    ///
    /// ```
    /// use tonic::{Status, Response};
    /// use tonic_richer_error::{WithErrorDetails};
    ///
    /// fn handle_req_result<T>(req_result: Result<Response<T>, Status>) {
    ///     match req_result {
    ///         Ok(_) => {},
    ///         Err(status) => {
    ///             let err_details = status.get_error_details().unwrap();
    ///             if let Some(bad_request) = err_details.bad_request {
    ///                 // deal with bad_request details
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    fn get_error_details(&self) -> Result<ErrorDetails, DecodeError>;

    /// Get a vector of `ErrorDetail` enums from a `tonic::Status`.
    /// # Examples
    ///
    /// ```
    /// use tonic::{Status, Response};
    /// use tonic_richer_error::{ErrorDetail, WithErrorDetails};
    ///
    /// fn handle_req_result<T>(req_result: Result<Response<T>, Status>) {
    ///     match req_result {
    ///         Ok(_) => {},
    ///         Err(status) => {
    ///             let err_details = status.get_error_details_vec().unwrap();
    ///             for (i, err_detail) in err_details.iter().enumerate() {
    ///                  match err_detail {
    ///                     ErrorDetail::BadRequest(bad_request) => {
    ///                         // deal with bad_request details
    ///                     }
    ///                     _ => {}
    ///                  }
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    fn get_error_details_vec(&self) -> Result<Vec<ErrorDetail>, DecodeError>;

    /// Get first `RetryInfo` details found on a `tonic::Status`.
    /// # Examples
    ///
    /// ```
    /// use tonic::{Status, Response};
    /// use tonic_richer_error::{WithErrorDetails};
    ///
    /// fn handle_req_result<T>(req_result: Result<Response<T>, Status>) {
    ///     match req_result {
    ///         Ok(_) => {},
    ///         Err(status) => {
    ///             if let Some(retry_info) = status.get_details_retry_info() {
    ///                 // deal with retry_info details
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    fn get_details_retry_info(&self) -> Option<RetryInfo>;

    /// Get first `DebugInfo` details found on a `tonic::Status`.
    /// # Examples
    ///
    /// ```
    /// use tonic::{Status, Response};
    /// use tonic_richer_error::{WithErrorDetails};
    ///
    /// fn handle_req_result<T>(req_result: Result<Response<T>, Status>) {
    ///     match req_result {
    ///         Ok(_) => {},
    ///         Err(status) => {
    ///             if let Some(debug_info) = status.get_details_debug_info() {
    ///                 // deal with debug_info details
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    fn get_details_debug_info(&self) -> Option<DebugInfo>;

    /// Get first `QuotaFailure` details found on a `tonic::Status`.
    /// # Examples
    ///
    /// ```
    /// use tonic::{Status, Response};
    /// use tonic_richer_error::{WithErrorDetails};
    ///
    /// fn handle_req_result<T>(req_result: Result<Response<T>, Status>) {
    ///     match req_result {
    ///         Ok(_) => {},
    ///         Err(status) => {
    ///             if let Some(quota_failure) = status.get_details_quota_failure() {
    ///                 // deal with quota_failure details
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    fn get_details_quota_failure(&self) -> Option<QuotaFailure>;

    /// Get first `ErrorInfo` details found on a `tonic::Status`.
    /// # Examples
    ///
    /// ```
    /// use tonic::{Status, Response};
    /// use tonic_richer_error::{WithErrorDetails};
    ///
    /// fn handle_req_result<T>(req_result: Result<Response<T>, Status>) {
    ///     match req_result {
    ///         Ok(_) => {},
    ///         Err(status) => {
    ///             if let Some(error_info) = status.get_details_error_info() {
    ///                 // deal with error_info details
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    fn get_details_error_info(&self) -> Option<ErrorInfo>;

    /// Get first `PreconditionFailure` details found on a `tonic::Status`.
    /// # Examples
    ///
    /// ```
    /// use tonic::{Status, Response};
    /// use tonic_richer_error::{WithErrorDetails};
    ///
    /// fn handle_req_result<T>(req_result: Result<Response<T>, Status>) {
    ///     match req_result {
    ///         Ok(_) => {},
    ///         Err(status) => {
    ///             if let Some(precondition_failure) = status.get_details_precondition_failure() {
    ///                 // deal with precondition_failure details
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    fn get_details_precondition_failure(&self) -> Option<PreconditionFailure>;

    /// Get first `BadRequest` details found on a `tonic::Status`.
    /// # Examples
    ///
    /// ```
    /// use tonic::{Status, Response};
    /// use tonic_richer_error::{WithErrorDetails};
    ///
    /// fn handle_req_result<T>(req_result: Result<Response<T>, Status>) {
    ///     match req_result {
    ///         Ok(_) => {},
    ///         Err(status) => {
    ///             if let Some(bad_request) = status.get_details_bad_request() {
    ///                 // deal with bad_request details
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    fn get_details_bad_request(&self) -> Option<BadRequest>;

    /// Get first `RequestInfo` details found on a `tonic::Status`.
    /// # Examples
    ///
    /// ```
    /// use tonic::{Status, Response};
    /// use tonic_richer_error::{WithErrorDetails};
    ///
    /// fn handle_req_result<T>(req_result: Result<Response<T>, Status>) {
    ///     match req_result {
    ///         Ok(_) => {},
    ///         Err(status) => {
    ///             if let Some(request_info) = status.get_details_request_info() {
    ///                 // deal with request_info details
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    fn get_details_request_info(&self) -> Option<RequestInfo>;

    /// Get first `ResourceInfo` details found on a `tonic::Status`.
    /// # Examples
    ///
    /// ```
    /// use tonic::{Status, Response};
    /// use tonic_richer_error::{WithErrorDetails};
    ///
    /// fn handle_req_result<T>(req_result: Result<Response<T>, Status>) {
    ///     match req_result {
    ///         Ok(_) => {},
    ///         Err(status) => {
    ///             if let Some(resource_info) = status.get_details_resource_info() {
    ///                 // deal with resource_info details
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    fn get_details_resource_info(&self) -> Option<ResourceInfo>;

    /// Get first `Help` details found on a `tonic::Status`
    /// # Examples
    ///
    /// ```
    /// use tonic::{Status, Response};
    /// use tonic_richer_error::{WithErrorDetails};
    ///
    /// fn handle_req_result<T>(req_result: Result<Response<T>, Status>) {
    ///     match req_result {
    ///         Ok(_) => {},
    ///         Err(status) => {
    ///             if let Some(help) = status.get_details_help() {
    ///                 // deal with help details
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    fn get_details_help(&self) -> Option<Help>;

    /// Get first `LocalizedMessage` details found on a `tonic::Status`.
    /// # Examples
    ///
    /// ```
    /// use tonic::{Status, Response};
    /// use tonic_richer_error::{WithErrorDetails};
    ///
    /// fn handle_req_result<T>(req_result: Result<Response<T>, Status>) {
    ///     match req_result {
    ///         Ok(_) => {},
    ///         Err(status) => {
    ///             if let Some(localized_message) = status.get_details_localized_message() {
    ///                 // deal with localized_message details
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    fn get_details_localized_message(&self) -> Option<LocalizedMessage>;
}

impl WithErrorDetails for Status {
    fn with_error_details(
        code: Code,
        message: impl Into<String>,
        details: ErrorDetails,
    ) -> Result<Self, EncodeError> {
        let message: String = message.into();

        let mut conv_details: Vec<Any> = Vec::with_capacity(10);

        if let Some(retry_info) = details.retry_info {
            conv_details.push(retry_info.into_any()?);
        }

        if let Some(debug_info) = details.debug_info {
            conv_details.push(debug_info.into_any()?);
        }

        if let Some(quota_failure) = details.quota_failure {
            conv_details.push(quota_failure.into_any()?);
        }

        if let Some(error_info) = details.error_info {
            conv_details.push(error_info.into_any()?);
        }

        if let Some(precondition_failure) = details.precondition_failure {
            conv_details.push(precondition_failure.into_any()?);
        }

        if let Some(bad_request) = details.bad_request {
            conv_details.push(bad_request.into_any()?);
        }

        if let Some(request_info) = details.request_info {
            conv_details.push(request_info.into_any()?);
        }

        if let Some(resource_info) = details.resource_info {
            conv_details.push(resource_info.into_any()?);
        }

        if let Some(help) = details.help {
            conv_details.push(help.into_any()?);
        }

        if let Some(localized_message) = details.localized_message {
            conv_details.push(localized_message.into_any()?);
        }

        let status = pb::Status {
            code: code as i32,
            message: message.clone(),
            details: conv_details,
        };

        let mut buf: Vec<u8> = Vec::new();
        buf.reserve(status.encoded_len());
        status.encode(&mut buf)?;

        let status = Status::with_details(code, message, Bytes::from(buf));

        Ok(status)
    }

    fn with_error_details_vec(
        code: Code,
        message: impl Into<String>,
        details: Vec<ErrorDetail>,
    ) -> Result<Self, EncodeError> {
        let message: String = message.into();

        let mut conv_details: Vec<Any> = Vec::with_capacity(details.len());

        for error_detail in details.into_iter() {
            match error_detail {
                ErrorDetail::RetryInfo(retry_info) => {
                    conv_details.push(retry_info.into_any()?);
                }
                ErrorDetail::DebugInfo(debug_info) => {
                    conv_details.push(debug_info.into_any()?);
                }
                ErrorDetail::QuotaFailure(quota_failure) => {
                    conv_details.push(quota_failure.into_any()?);
                }
                ErrorDetail::ErrorInfo(error_info) => {
                    conv_details.push(error_info.into_any()?);
                }
                ErrorDetail::PreconditionFailure(prec_failure) => {
                    conv_details.push(prec_failure.into_any()?);
                }
                ErrorDetail::BadRequest(bad_req) => {
                    conv_details.push(bad_req.into_any()?);
                }
                ErrorDetail::RequestInfo(req_info) => {
                    conv_details.push(req_info.into_any()?);
                }
                ErrorDetail::ResourceInfo(res_info) => {
                    conv_details.push(res_info.into_any()?);
                }
                ErrorDetail::Help(help) => {
                    conv_details.push(help.into_any()?);
                }
                ErrorDetail::LocalizedMessage(loc_message) => {
                    conv_details.push(loc_message.into_any()?);
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

        let status = Status::with_details(code, message, Bytes::from(buf));

        Ok(status)
    }

    fn get_error_details(&self) -> Result<ErrorDetails, DecodeError> {
        let status = pb::Status::decode(self.details())?;

        let mut details = ErrorDetails::new();

        for any in status.details.into_iter() {
            match any.type_url.as_str() {
                RetryInfo::TYPE_URL => {
                    details.retry_info = Some(RetryInfo::from_any(any)?);
                }
                DebugInfo::TYPE_URL => {
                    details.debug_info = Some(DebugInfo::from_any(any)?);
                }
                QuotaFailure::TYPE_URL => {
                    details.quota_failure = Some(QuotaFailure::from_any(any)?);
                }
                ErrorInfo::TYPE_URL => {
                    details.error_info = Some(ErrorInfo::from_any(any)?);
                }
                PreconditionFailure::TYPE_URL => {
                    details.precondition_failure = Some(PreconditionFailure::from_any(any)?);
                }
                BadRequest::TYPE_URL => {
                    details.bad_request = Some(BadRequest::from_any(any)?);
                }
                RequestInfo::TYPE_URL => {
                    details.request_info = Some(RequestInfo::from_any(any)?);
                }
                ResourceInfo::TYPE_URL => {
                    details.resource_info = Some(ResourceInfo::from_any(any)?);
                }
                Help::TYPE_URL => {
                    details.help = Some(Help::from_any(any)?);
                }
                LocalizedMessage::TYPE_URL => {
                    details.localized_message = Some(LocalizedMessage::from_any(any)?);
                }
                _ => {}
            }
        }

        Ok(details)
    }

    fn get_error_details_vec(&self) -> Result<Vec<ErrorDetail>, DecodeError> {
        let status = pb::Status::decode(self.details())?;

        let mut details: Vec<ErrorDetail> = Vec::with_capacity(status.details.len());

        for any in status.details.into_iter() {
            match any.type_url.as_str() {
                RetryInfo::TYPE_URL => {
                    details.push(RetryInfo::from_any(any)?.into());
                }
                DebugInfo::TYPE_URL => {
                    details.push(DebugInfo::from_any(any)?.into());
                }
                QuotaFailure::TYPE_URL => {
                    details.push(QuotaFailure::from_any(any)?.into());
                }
                ErrorInfo::TYPE_URL => {
                    details.push(ErrorInfo::from_any(any)?.into());
                }
                PreconditionFailure::TYPE_URL => {
                    details.push(PreconditionFailure::from_any(any)?.into());
                }
                BadRequest::TYPE_URL => {
                    details.push(BadRequest::from_any(any)?.into());
                }
                RequestInfo::TYPE_URL => {
                    details.push(RequestInfo::from_any(any)?.into());
                }
                ResourceInfo::TYPE_URL => {
                    details.push(ResourceInfo::from_any(any)?.into());
                }
                Help::TYPE_URL => {
                    details.push(Help::from_any(any)?.into());
                }
                LocalizedMessage::TYPE_URL => {
                    details.push(LocalizedMessage::from_any(any)?.into());
                }
                _ => {}
            }
        }

        Ok(details)
    }

    fn get_details_retry_info(&self) -> Option<RetryInfo> {
        let status = pb::Status::decode(self.details()).ok()?;

        for any in status.details.into_iter() {
            match any.type_url.as_str() {
                RetryInfo::TYPE_URL => match RetryInfo::from_any(any) {
                    Ok(detail) => return Some(detail),
                    Err(_) => {}
                },
                _ => {}
            }
        }

        None
    }

    fn get_details_debug_info(&self) -> Option<DebugInfo> {
        let status = pb::Status::decode(self.details()).ok()?;

        for any in status.details.into_iter() {
            match any.type_url.as_str() {
                DebugInfo::TYPE_URL => match DebugInfo::from_any(any) {
                    Ok(detail) => return Some(detail),
                    Err(_) => {}
                },
                _ => {}
            }
        }

        None
    }

    fn get_details_quota_failure(&self) -> Option<QuotaFailure> {
        let status = pb::Status::decode(self.details()).ok()?;

        for any in status.details.into_iter() {
            match any.type_url.as_str() {
                QuotaFailure::TYPE_URL => match QuotaFailure::from_any(any) {
                    Ok(detail) => return Some(detail),
                    Err(_) => {}
                },
                _ => {}
            }
        }

        None
    }

    fn get_details_error_info(&self) -> Option<ErrorInfo> {
        let status = pb::Status::decode(self.details()).ok()?;

        for any in status.details.into_iter() {
            match any.type_url.as_str() {
                ErrorInfo::TYPE_URL => match ErrorInfo::from_any(any) {
                    Ok(detail) => return Some(detail),
                    Err(_) => {}
                },
                _ => {}
            }
        }

        None
    }

    fn get_details_precondition_failure(&self) -> Option<PreconditionFailure> {
        let status = pb::Status::decode(self.details()).ok()?;

        for any in status.details.into_iter() {
            match any.type_url.as_str() {
                PreconditionFailure::TYPE_URL => match PreconditionFailure::from_any(any) {
                    Ok(detail) => return Some(detail),
                    Err(_) => {}
                },
                _ => {}
            }
        }

        None
    }

    fn get_details_bad_request(&self) -> Option<BadRequest> {
        let status = pb::Status::decode(self.details()).ok()?;

        for any in status.details.into_iter() {
            match any.type_url.as_str() {
                BadRequest::TYPE_URL => match BadRequest::from_any(any) {
                    Ok(detail) => return Some(detail),
                    Err(_) => {}
                },
                _ => {}
            }
        }

        None
    }

    fn get_details_request_info(&self) -> Option<RequestInfo> {
        let status = pb::Status::decode(self.details()).ok()?;

        for any in status.details.into_iter() {
            match any.type_url.as_str() {
                RequestInfo::TYPE_URL => match RequestInfo::from_any(any) {
                    Ok(detail) => return Some(detail),
                    Err(_) => {}
                },
                _ => {}
            }
        }

        None
    }

    fn get_details_resource_info(&self) -> Option<ResourceInfo> {
        let status = pb::Status::decode(self.details()).ok()?;

        for any in status.details.into_iter() {
            match any.type_url.as_str() {
                ResourceInfo::TYPE_URL => match ResourceInfo::from_any(any) {
                    Ok(detail) => return Some(detail),
                    Err(_) => {}
                },
                _ => {}
            }
        }

        None
    }

    fn get_details_help(&self) -> Option<Help> {
        let status = pb::Status::decode(self.details()).ok()?;

        for any in status.details.into_iter() {
            match any.type_url.as_str() {
                Help::TYPE_URL => match Help::from_any(any) {
                    Ok(detail) => return Some(detail),
                    Err(_) => {}
                },
                _ => {}
            }
        }

        None
    }

    fn get_details_localized_message(&self) -> Option<LocalizedMessage> {
        let status = pb::Status::decode(self.details()).ok()?;

        for any in status.details.into_iter() {
            match any.type_url.as_str() {
                LocalizedMessage::TYPE_URL => match LocalizedMessage::from_any(any) {
                    Ok(detail) => return Some(detail),
                    Err(_) => {}
                },
                _ => {}
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::time::Duration;
    use tonic::{Code, Status};

    use super::{
        BadRequest, DebugInfo, ErrorDetails, ErrorInfo, Help, LocalizedMessage,
        PreconditionFailure, QuotaFailure, RequestInfo, ResourceInfo, RetryInfo, WithErrorDetails,
    };

    #[test]
    fn gen_status_with_details() {
        let mut metadata = HashMap::new();
        metadata.insert("limitPerRequest".to_string(), "100".into());

        let mut err_details = ErrorDetails::new();

        err_details
            .set_retry_info(Some(Duration::from_secs(5)))
            .set_debug_info(
                vec![
                    "trace3".to_string(),
                    "trace2".to_string(),
                    "trace1".to_string(),
                ],
                "details",
            )
            .add_quota_failure_violation("clientip:<ip address>", "description")
            .set_error_info("SOME_INFO", "example.local", metadata.clone())
            .add_precondition_failure_violation("TOS", "example.local", "description")
            .add_bad_request_violation("field", "description")
            .set_request_info("request-id", "some-request-data")
            .set_resource_info("resource-type", "resource-name", "owner", "description")
            .add_help_link("link to resource", "resource.example.local")
            .set_localized_message("en-US", "message for the user");

        let fmt_details = format!("{:?}", err_details);

        println!("{fmt_details}\n");

        let err_details_vec = vec![
            RetryInfo::new(Some(Duration::from_secs(5))).into(),
            DebugInfo::new(
                vec![
                    "trace3".to_string(),
                    "trace2".to_string(),
                    "trace1".to_string(),
                ],
                "details",
            )
            .into(),
            QuotaFailure::with_violation("clientip:<ip address>", "description").into(),
            ErrorInfo::new("SOME_INFO", "example.local", metadata).into(),
            PreconditionFailure::with_violation("TOS", "example.local", "description").into(),
            BadRequest::with_violation("field", "description").into(),
            RequestInfo::new("request-id", "some-request-data").into(),
            ResourceInfo::new("resource-type", "resource-name", "owner", "description").into(),
            Help::with_link("link to resource", "resource.example.local").into(),
            LocalizedMessage::new("en-US", "message for the user").into(),
        ];

        let fmt_details_vec = format!("{:?}", err_details_vec);

        println!("{fmt_details_vec}\n");

        let status_from_struct = match Status::with_error_details(
            Code::InvalidArgument,
            "error with bad request details",
            err_details,
        ) {
            Ok(status) => status,
            Err(err) => panic!("Error generating status: {:?}", err),
        };

        let fmt_status_with_details = format!("{:?}", status_from_struct);

        println!("{:?}\n", fmt_status_with_details);

        let status_from_vec = match Status::with_error_details_vec(
            Code::InvalidArgument,
            "error with bad request details",
            err_details_vec,
        ) {
            Ok(status) => status,
            Err(err) => panic!("Error generating status: {:?}", err),
        };

        let fmt_status_with_details_vec = format!("{:?}", status_from_vec);

        println!("{:?}\n", fmt_status_with_details_vec);

        let ext_details = match status_from_vec.get_error_details() {
            Ok(ext_details) => ext_details,
            Err(err) => panic!(
                "Error extracting details struct from status_from_vec: {:?}",
                err
            ),
        };

        let fmt_ext_details = format!("{:?}", ext_details);

        println!("{:?}\n", ext_details.debug_info);
        println!("{fmt_ext_details}\n");

        assert!(
            fmt_ext_details.eq(&fmt_details),
            "Extracted details struct differs from original details struct"
        );

        let ext_details_vec = match status_from_struct.get_error_details_vec() {
            Ok(ext_details) => ext_details,
            Err(err) => panic!(
                "Error extracting details_vec from status_from_struct: {:?}",
                err
            ),
        };

        let fmt_ext_details_vec = format!("{:?}", ext_details_vec);

        println!("fmt_ext_details_vec: {:?}\n", fmt_ext_details_vec);

        assert!(
            fmt_ext_details_vec.eq(&fmt_details_vec),
            "Extracted details vec differs from original details vec"
        );
    }
}
