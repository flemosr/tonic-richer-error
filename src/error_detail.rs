mod retry_info;

pub use retry_info::RetryInfo;

mod debug_info;

pub use debug_info::DebugInfo;

mod quota_failure;

pub use quota_failure::QuotaFailure;

mod error_info;

pub use error_info::ErrorInfo;

mod prec_failure;

pub use prec_failure::PreconditionFailure;

mod bad_request;

pub use bad_request::BadRequest;

mod request_info;

pub use request_info::RequestInfo;

#[derive(Debug)]
pub enum ErrorDetail {
    RetryInfo(RetryInfo),
    DebugInfo(DebugInfo),
    QuotaFailure(QuotaFailure),
    ErrorInfo(ErrorInfo),
    PreconditionFailure(PreconditionFailure),
    BadRequest(BadRequest),
    RequestInfo(RequestInfo),
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

impl From<PreconditionFailure> for ErrorDetail {
    fn from(err_detail: PreconditionFailure) -> Self {
        ErrorDetail::PreconditionFailure(err_detail)
    }
}

impl From<BadRequest> for ErrorDetail {
    fn from(err_detail: BadRequest) -> Self {
        ErrorDetail::BadRequest(err_detail)
    }
}

impl From<RequestInfo> for ErrorDetail {
    fn from(err_detail: RequestInfo) -> Self {
        ErrorDetail::RequestInfo(err_detail)
    }
}
