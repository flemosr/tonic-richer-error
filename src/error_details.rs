use std::{collections::HashMap, time};

use super::error_detail::*;

#[derive(Clone, Debug)]
pub struct ErrorDetails {
    pub retry_info: Option<RetryInfo>,
    pub debug_info: Option<DebugInfo>,
    pub quota_failure: Option<QuotaFailure>,
    pub error_info: Option<ErrorInfo>,
    pub precondition_failure: Option<PreconditionFailure>,
    pub bad_request: Option<BadRequest>,
    pub request_info: Option<RequestInfo>,
    pub resource_info: Option<ResourceInfo>,
    pub help: Option<Help>,
    pub localized_message: Option<LocalizedMessage>,
}

impl ErrorDetails {
    pub fn new() -> Self {
        ErrorDetails {
            retry_info: None,
            debug_info: None,
            quota_failure: None,
            error_info: None,
            precondition_failure: None,
            bad_request: None,
            request_info: None,
            resource_info: None,
            help: None,
            localized_message: None,
        }
    }

    pub fn with_retry_info(retry_delay: Option<time::Duration>) -> Self {
        ErrorDetails {
            retry_info: Some(RetryInfo::new(retry_delay)),
            ..ErrorDetails::new()
        }
    }

    pub fn with_debug_info(stack_entries: Vec<String>, detail: impl Into<String>) -> Self {
        ErrorDetails {
            debug_info: Some(DebugInfo::new(stack_entries, detail)),
            ..ErrorDetails::new()
        }
    }

    pub fn with_quota_failure(violations: Vec<QuotaViolation>) -> Self {
        ErrorDetails {
            quota_failure: Some(QuotaFailure::new(violations)),
            ..ErrorDetails::new()
        }
    }

    pub fn with_quota_failure_violation(
        subject: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        ErrorDetails {
            quota_failure: Some(QuotaFailure::with_violation(subject, description)),
            ..ErrorDetails::new()
        }
    }

    pub fn with_error_info(
        reason: impl Into<String>,
        domain: impl Into<String>,
        metadata: HashMap<String, String>,
    ) -> Self {
        ErrorDetails {
            error_info: Some(ErrorInfo::new(reason, domain, metadata)),
            ..ErrorDetails::new()
        }
    }

    pub fn with_precondition_failure(violations: Vec<PreconditionViolation>) -> Self {
        ErrorDetails {
            precondition_failure: Some(PreconditionFailure::new(violations)),
            ..ErrorDetails::new()
        }
    }

    pub fn with_precondition_failure_violation(
        violation_type: impl Into<String>,
        subject: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        ErrorDetails {
            precondition_failure: Some(PreconditionFailure::with_violation(
                violation_type,
                subject,
                description,
            )),
            ..ErrorDetails::new()
        }
    }

    pub fn with_bad_request(field_violations: Vec<FieldViolation>) -> Self {
        ErrorDetails {
            bad_request: Some(BadRequest::new(field_violations)),
            ..ErrorDetails::new()
        }
    }

    pub fn with_bad_request_violation(
        field: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        ErrorDetails {
            bad_request: Some(BadRequest::with_violation(field, description)),
            ..ErrorDetails::new()
        }
    }

    pub fn with_request_info(
        request_id: impl Into<String>,
        serving_data: impl Into<String>,
    ) -> Self {
        ErrorDetails {
            request_info: Some(RequestInfo::new(request_id, serving_data)),
            ..ErrorDetails::new()
        }
    }

    pub fn with_resource_info(
        resource_type: impl Into<String>,
        resource_name: impl Into<String>,
        owner: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        ErrorDetails {
            resource_info: Some(ResourceInfo::new(
                resource_type,
                resource_name,
                owner,
                description,
            )),
            ..ErrorDetails::new()
        }
    }

    pub fn with_help(links: Vec<HelpLink>) -> Self {
        ErrorDetails {
            help: Some(Help::new(links)),
            ..ErrorDetails::new()
        }
    }

    pub fn with_help_link(description: impl Into<String>, url: impl Into<String>) -> Self {
        ErrorDetails {
            help: Some(Help::with_link(description, url)),
            ..ErrorDetails::new()
        }
    }

    pub fn with_localized_message(locale: impl Into<String>, message: impl Into<String>) -> Self {
        ErrorDetails {
            localized_message: Some(LocalizedMessage::new(locale, message)),
            ..ErrorDetails::new()
        }
    }
}

impl ErrorDetails {
    pub fn set_retry_info(&mut self, retry_delay: Option<time::Duration>) -> &mut Self {
        self.retry_info = Some(RetryInfo::new(retry_delay));
        self
    }

    pub fn set_debug_info(
        &mut self,
        stack_entries: Vec<String>,
        detail: impl Into<String>,
    ) -> &mut Self {
        self.debug_info = Some(DebugInfo::new(stack_entries, detail));
        self
    }

    pub fn set_quota_failure(&mut self, violations: Vec<QuotaViolation>) -> &mut Self {
        self.quota_failure = Some(QuotaFailure::new(violations));
        self
    }

    pub fn add_quota_failure_violation(
        &mut self,
        subject: impl Into<String>,
        description: impl Into<String>,
    ) -> &mut Self {
        match &mut self.quota_failure {
            Some(quota_failure) => {
                quota_failure.add_violation(subject, description);
            }
            None => {
                self.quota_failure = Some(QuotaFailure::with_violation(subject, description));
            }
        };
        self
    }

    pub fn has_quota_failure_violation(&self) -> bool {
        if let Some(quota_failure) = &self.quota_failure {
            return !quota_failure.violations.is_empty();
        }
        false
    }

    pub fn set_error_info(
        &mut self,
        reason: impl Into<String>,
        domain: impl Into<String>,
        metadata: HashMap<String, String>,
    ) -> &mut Self {
        self.error_info = Some(ErrorInfo::new(reason, domain, metadata));
        self
    }

    pub fn set_precondition_failure(
        &mut self,
        violations: Vec<PreconditionViolation>,
    ) -> &mut Self {
        self.precondition_failure = Some(PreconditionFailure::new(violations));
        self
    }

    pub fn add_precondition_failure_violation(
        &mut self,
        violation_type: impl Into<String>,
        subject: impl Into<String>,
        description: impl Into<String>,
    ) -> &mut Self {
        match &mut self.precondition_failure {
            Some(precondition_failure) => {
                precondition_failure.add_violation(violation_type, subject, description);
            }
            None => {
                self.precondition_failure = Some(PreconditionFailure::with_violation(
                    violation_type,
                    subject,
                    description,
                ));
            }
        };
        self
    }

    pub fn has_precondition_failure_violation(&self) -> bool {
        if let Some(precondition_failure) = &self.precondition_failure {
            return !precondition_failure.violations.is_empty();
        }
        false
    }

    pub fn set_bad_request(&mut self, violations: Vec<FieldViolation>) -> &mut Self {
        self.bad_request = Some(BadRequest::new(violations));
        self
    }

    pub fn add_bad_request_violation(
        &mut self,
        field: impl Into<String>,
        description: impl Into<String>,
    ) -> &mut Self {
        match &mut self.bad_request {
            Some(bad_request) => {
                bad_request.add_violation(field, description);
            }
            None => {
                self.bad_request = Some(BadRequest::with_violation(field, description));
            }
        };
        self
    }

    pub fn has_bad_request_violation(&self) -> bool {
        if let Some(bad_request) = &self.bad_request {
            return !bad_request.field_violations.is_empty();
        }
        false
    }

    pub fn set_request_info(
        &mut self,
        request_id: impl Into<String>,
        serving_data: impl Into<String>,
    ) -> &mut Self {
        self.request_info = Some(RequestInfo::new(request_id, serving_data));
        self
    }

    pub fn set_resource_info(
        &mut self,
        resource_type: impl Into<String>,
        resource_name: impl Into<String>,
        owner: impl Into<String>,
        description: impl Into<String>,
    ) -> &mut Self {
        self.resource_info = Some(ResourceInfo::new(
            resource_type,
            resource_name,
            owner,
            description,
        ));
        self
    }

    pub fn set_help(&mut self, links: Vec<HelpLink>) -> &mut Self {
        self.help = Some(Help::new(links));
        self
    }

    pub fn add_help_link(
        &mut self,
        description: impl Into<String>,
        url: impl Into<String>,
    ) -> &mut Self {
        match &mut self.help {
            Some(help) => {
                help.add_link(description, url);
            }
            None => {
                self.help = Some(Help::with_link(description, url));
            }
        };
        self
    }

    pub fn has_help_link(&self) -> bool {
        if let Some(help) = &self.help {
            return !help.links.is_empty();
        }
        false
    }

    pub fn set_localized_message(
        &mut self,
        locale: impl Into<String>,
        message: impl Into<String>,
    ) -> &mut Self {
        self.localized_message = Some(LocalizedMessage::new(locale, message));
        self
    }
}
