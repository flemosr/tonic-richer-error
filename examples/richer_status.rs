use tonic::{Code, Status};
use tonic_richer_error::{BadRequest, ErrorDetail, WithErrorDetails};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut br_details = BadRequest::empty();

    br_details
        .add_violation("field_a", "description_a")
        .add_violation("field_b", "description_b");

    let status = Status::with_error_details(
        Code::InvalidArgument,
        "error with bad request details",
        vec![br_details],
    );

    println!("{:?}", status);

    let err_details = status.extract_error_details().unwrap_or(vec![]);

    for (i, err_detail) in err_details.iter().enumerate() {
        println!("err_detail[{i}]");
        match err_detail {
            ErrorDetail::BadRequest(bad_req) => {
                println!(" {:?}", bad_req);
                // deal with bad_req error details
            }
        }
    }

    Ok(())
}
