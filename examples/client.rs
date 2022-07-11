mod pb {
    include!("./pb/schedule.rs");
}

use pb::schedule_client::ScheduleClient;
use pb::DayInfoReq;

use tonic_richer_error::WithErrorDetails;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ScheduleClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(DayInfoReq {
        // day_code: "invalid code".into(),
        day_code: "aaa".into(),
        // day_code: "mon".into(),
    });

    let response = match client.day_info(request).await {
        Ok(response) => response,
        Err(status) => {
            let err_details = status.get_error_details().unwrap();

            if let Some(bad_request) = err_details.bad_request {
                // deal with bad_request details
                println!(" {:?}", bad_request);
            }
            if let Some(help) = err_details.help {
                // deal with help details
                println!(" {:?}", help);
            }
            if let Some(localized_message) = err_details.localized_message {
                // deal with localized_message details
                println!(" {:?}", localized_message);
            }

            return Ok(());
        }
    };

    println!("RESPONSE={:?}", response);

    Ok(())
}
