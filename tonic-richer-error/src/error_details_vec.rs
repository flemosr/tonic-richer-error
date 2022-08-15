use super::std_messages::*;

/// Wraps the structs corresponding to the standard error messages, allowing
/// the implementation and handling of vectors containing any of them.
#[derive(Clone, Debug)]
pub enum ErrorDetail {
    /// Wraps data corresponding to the `RetryInfo` standard error message.
    RetryInfo(RetryInfo),

    /// Wraps data corresponding to the `DebugInfo` standard error message.
    DebugInfo(DebugInfo),

    /// Wraps data corresponding to the `QuotaFailure` standard error message.
    QuotaFailure(QuotaFailure),

    /// Wraps data corresponding to the `ErrorInfo` standard error message.
    ErrorInfo(ErrorInfo),

    /// Wraps data corresponding to the `PreconditionFailure` standard error message.
    PreconditionFailure(PreconditionFailure),

    /// Wraps data corresponding to the `BadRequest` standard error message.
    BadRequest(BadRequest),

    /// Wraps data corresponding to the `RequestInfo` standard error message.
    RequestInfo(RequestInfo),

    /// Wraps data corresponding to the `ResourceInfo` standard error message.
    ResourceInfo(ResourceInfo),

    /// Wraps data corresponding to the `Help` standard error message.
    Help(Help),

    /// Wraps data corresponding to the `LocalizedMessage` standard error message.
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
