mod retry_info;

pub use retry_info::RetryInfo;

mod debug_info;

pub use debug_info::DebugInfo;

mod quota_failure;

pub use quota_failure::QuotaFailure;

mod error_info;

pub use error_info::ErrorInfo;

mod bad_request;

pub use bad_request::BadRequest;

#[derive(Debug)]
pub enum ErrorDetail {
    RetryInfo(RetryInfo),
    DebugInfo(DebugInfo),
    QuotaFailure(QuotaFailure),
    ErrorInfo(ErrorInfo),
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

impl From<ErrorInfo> for ErrorDetail {
    fn from(err_detail: ErrorInfo) -> Self {
        ErrorDetail::ErrorInfo(err_detail)
    }
}

impl From<BadRequest> for ErrorDetail {
    fn from(err_detail: BadRequest) -> Self {
        ErrorDetail::BadRequest(err_detail)
    }
}