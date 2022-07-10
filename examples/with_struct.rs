use std::collections::HashMap;
use std::time::Duration;
use tonic::{Code, Status};
use tonic_richer_error::{ErrorDetails, WithErrorDetails};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert("instanceLimitPerRequest".into(), "100".into());

    let mut err_details = ErrorDetails::new();

    err_details
        .set_retry_info(Some(Duration::from_secs(5)))
        .set_debug_info(
            vec!["trace3".into(), "trace2".into(), "trace1".into()],
            "details",
        )
        .add_quota_failure_violation("clientip:<ip address>", "description")
        .set_error_info("SOME_INFO", "mydomain.com", metadata)
        .add_precondition_failure_violation("TOS", "example.local", "Terms of service not accepted")
        .add_precondition_failure_violation("FNF", "example.local", "File not found")
        .add_bad_request_violation("field_1", "description of why field_1 value is invalid")
        .add_bad_request_violation("field_2", "description of why field_2 value is invalid")
        .set_request_info("request-id", "some-req-data")
        .set_resource_info("resource-type", "resource-name", "owner", "description")
        .add_help_link("link to resource a", "resource-a.example.local")
        .add_help_link("link to resource b", "resource-b.example.local")
        .set_localized_message("en-US", "message for the user");

    let status =
        Status::with_error_details(Code::InvalidArgument, "BAD_REQUEST", err_details).unwrap();

    let err_details = status.get_error_details().unwrap_or(ErrorDetails::new());

    if let Some(retry_info) = err_details.retry_info {
        println!(" {:?}", retry_info);
        // deal with retry_info details
    }

    if let Some(debug_info) = err_details.debug_info {
        println!(" {:?}", debug_info);
        // deal with debug_info details
    }

    if let Some(quota_failure) = err_details.quota_failure {
        println!(" {:?}", quota_failure);
        // deal with quota_failure details
    }

    if let Some(error_info) = err_details.error_info {
        println!(" {:?}", error_info);
        // deal with error_info details
    }

    if let Some(precondition_failure) = err_details.precondition_failure {
        println!(" {:?}", precondition_failure);
        // deal with precondition_failure details
    }

    if let Some(bad_request) = err_details.bad_request {
        println!(" {:?}", bad_request);
        // deal with bad_request details
    }

    if let Some(request_info) = err_details.request_info {
        println!(" {:?}", request_info);
        // deal with request_info details
    }

    if let Some(resource_info) = err_details.resource_info {
        println!(" {:?}", resource_info);
        // deal with resource_info details
    }

    if let Some(help) = err_details.help {
        println!(" {:?}", help);
        // deal with help details
    }

    if let Some(localized_message) = err_details.localized_message {
        println!(" {:?}", localized_message);
        // deal with localized_message details
    }

    Ok(())
}
