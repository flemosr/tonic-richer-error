use super::error_detail::*;

/// Wraps the structs corresponding to the standard set of error message types,
/// to allow the implementation and handling of vectors containing any of them.
#[derive(Clone, Debug)]
pub enum ErrorDetail {
    RetryInfo(RetryInfo),
    DebugInfo(DebugInfo),
    QuotaFailure(QuotaFailure),
    ErrorInfo(ErrorInfo),
    PreconditionFailure(PreconditionFailure),
    BadRequest(BadRequest),
    RequestInfo(RequestInfo),
    ResourceInfo(ResourceInfo),
    Help(Help),
    LocalizedMessage(LocalizedMessage),
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

impl From<ResourceInfo> for ErrorDetail {
    fn from(err_detail: ResourceInfo) -> Self {
        ErrorDetail::ResourceInfo(err_detail)
    }
}

impl From<Help> for ErrorDetail {
    fn from(err_detail: Help) -> Self {
        ErrorDetail::Help(err_detail)
    }
}

impl From<LocalizedMessage> for ErrorDetail {
    fn from(err_detail: LocalizedMessage) -> Self {
        ErrorDetail::LocalizedMessage(err_detail)
    }
}
