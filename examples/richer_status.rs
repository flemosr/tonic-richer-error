use std::collections::HashMap;
use std::time::Duration;
use tonic::{Code, Status};
use tonic_richer_error::{
    BadRequest, DebugInfo, ErrorDetail, ErrorInfo, QuotaFailure, RequestInfo, RetryInfo,
    WithErrorDetails,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let retry_info = RetryInfo::with_retry_delay(Duration::from_secs(5));

    let debug_info = DebugInfo::with_stack(vec!["trace3", "trace2", "trace1"], "details");

    let quota_failure = QuotaFailure::with_violation("clientip:<ip address>", "description");

    let mut metadata = HashMap::new();
    metadata.insert("instanceLimitPerRequest", "100");

    let error_info = ErrorInfo::with_data("SOME_INFO", "mydomain.com", metadata);

    let mut br_details = BadRequest::empty();

    if true {
        br_details
            .add_violation("field_1", "description of why value is invalid")
            .add_violation("field_2", "description of why value is invalid")
            .add_violation("field", "description of why value is invalid")
            .add_violation("field", "description of why value is invalid")
            .add_violation("field", "description of why value is invalid");
    }

    let req_info = RequestInfo::with_data("request-id", "some-req-data");
    // let st = Status::internal("acont4cirf bshrf bwhbr");

    let status = Status::with_error_details(
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

    let err_details = status.extract_error_details().unwrap_or(vec![]);

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
