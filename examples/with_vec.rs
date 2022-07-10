use std::collections::HashMap;
use std::time::Duration;
use tonic::{Code, Status};
use tonic_richer_error::{
    BadRequest, DebugInfo, ErrorDetail, ErrorInfo, QuotaFailure, RequestInfo, RetryInfo,
    WithErrorDetails,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let retry_info = RetryInfo::new(Some(Duration::from_secs(5)));

    let debug_info = DebugInfo::new(
        vec![
            "trace3".to_string(),
            "trace2".to_string(),
            "trace1".to_string(),
        ],
        "details",
    );

    let quota_failure = QuotaFailure::with_violation("clientip:<ip address>", "description");

    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert("instanceLimitPerRequest".into(), "100".into());

    let error_info = ErrorInfo::new("SOME_INFO", "mydomain.com", metadata);

    let mut br_details = BadRequest::new(Vec::new());

    if true {
        br_details
            .add_violation("field_1", "description of why value is invalid")
            .add_violation("field_2", "description of why value is invalid");
    }

    let req_info = RequestInfo::new("request-id", "some-req-data");

    let status = Status::with_error_details_vec(
        Code::InvalidArgument,
        "BAD_REQUEST",
        vec![
            retry_info.into(),
            debug_info.into(),
            quota_failure.into(),
            error_info.into(),
            br_details.into(),
            req_info.into(),
        ],
    )
    .unwrap();

    println!("{:?}", status);

    let err_details = status.get_error_details_vec().unwrap_or(vec![]);

    for (i, err_detail) in err_details.iter().enumerate() {
        println!("err_detail[{i}]");
        match err_detail {
            ErrorDetail::RetryInfo(retry_info) => {
                println!(" {:?}", retry_info);
                // deal with retry_info details
            }
            ErrorDetail::DebugInfo(debug_info) => {
                println!(" {:?}", debug_info);
                // deal with debug_info details
            }
            ErrorDetail::QuotaFailure(quota_failure) => {
                println!(" {:?}", quota_failure);
                // deal with quota_failure details
            }
            ErrorDetail::ErrorInfo(error_info) => {
                println!(" {:?}", error_info);
                // deal with error_info details
            }
            ErrorDetail::PreconditionFailure(prec_failure) => {
                println!(" {:?}", prec_failure);
                // deal with prec_failure details
            }
            ErrorDetail::BadRequest(bad_request) => {
                println!(" {:?}", bad_request);
                // deal with bad_request details
            }
            ErrorDetail::RequestInfo(req_info) => {
                println!(" {:?}", req_info);
                // deal with req_info details
            }
            ErrorDetail::ResourceInfo(res_info) => {
                println!(" {:?}", res_info);
                // deal with res_info details
            }
            ErrorDetail::Help(help) => {
                println!(" {:?}", help);
                // deal with help details
            }
            ErrorDetail::LocalizedMessage(loc_message) => {
                println!(" {:?}", loc_message);
                // deal with loc_message details
            }
        }
    }

    Ok(())
}
