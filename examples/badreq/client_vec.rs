mod pb {
    include!("./pb/schedule.rs");
}

use pb::schedule_client::ScheduleClient;
use pb::DayInfoReq;

use tonic_richer_error::{ErrorDetail, WithErrorDetails};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ScheduleClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(DayInfoReq {
        day_code: "invalid code".into(),
        // day_code: "aaa".into(),
        // day_code: "mon".into(),
    });

    println!("\n Making request...");

    let response = match client.day_info(request).await {
        Ok(response) => response,
        Err(status) => {
            println!(" Error status received. Extracting error details...\n");

            let err_details = status.get_error_details_vec().unwrap();

            for (i, err_detail) in err_details.iter().enumerate() {
                println!("err_detail[{i}]");
                match err_detail {
                    ErrorDetail::BadRequest(bad_request) => {
                        // deal with bad_request details
                        println!(" {:?}", bad_request);
                    }
                    ErrorDetail::Help(help) => {
                        // deal with help details
                        println!(" {:?}", help);
                    }
                    ErrorDetail::LocalizedMessage(localized_message) => {
                        // deal with localized_message details
                        println!(" {:?}", localized_message);
                    }
                    _ => {
                        // ignore different error details
                    }
                }
            }

            println!("");

            return Ok(());
        }
    };

    println!(" Successfull response received.\n\n {:?}\n", response);

    Ok(())
}
