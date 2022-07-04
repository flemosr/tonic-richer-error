use std::time::Duration;
use tonic::{Code, Status};
use tonic_richer_error::{BadRequest, DebugInfo, ErrorDetail, RetryInfo, WithErrorDetails};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let retry_info = RetryInfo::with_retry_delay(Duration::from_secs(5));

    let debug_info = DebugInfo::with_stack(vec!["trace3", "trace2", "trace1"], "details");

    let mut br_details = BadRequest::empty();

    if true {
        br_details.add_violation("field", "description of why value is invalid");
    }

    let status = Status::with_error_details(
        Code::InvalidArgument,
        "error with bad request details",
        vec![retry_info.into(), debug_info.into(), br_details.into()],
    )
    .unwrap();

    println!("{:?}", status);

    let err_details = status.extract_error_details().unwrap_or(vec![]);

    for (i, err_detail) in err_details.iter().enumerate() {
        println!("err_detail[{i}]");
        match err_detail {
            ErrorDetail::RetryInfo(retry_info) => {
                println!(" {:?}", retry_info);
                // deal with retry_info error details
            }
            ErrorDetail::DebugInfo(debug_info) => {
                println!(" {:?}", debug_info);
                // deal with debug_info error details
            }
            ErrorDetail::BadRequest(bad_req) => {
                println!(" {:?}", bad_req);
                // deal with bad_req error details
            }
        }
    }

    Ok(())
}
