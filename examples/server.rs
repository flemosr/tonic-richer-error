use tonic::{transport::Server, Code, Request, Response, Status};
use tonic_richer_error::{ErrorDetails, WithErrorDetails};

mod pb {
    include!("./pb/schedule.rs");
}

use pb::schedule_server::{Schedule, ScheduleServer};
use pb::{DayInfoReq, DayInfoRes};

#[derive(Debug, Default)]
pub struct MySchedule {}

#[tonic::async_trait]
impl Schedule for MySchedule {
    async fn day_info(&self, request: Request<DayInfoReq>) -> Result<Response<DayInfoRes>, Status> {
        let mut err_details = ErrorDetails::new();

        println!("Got a request: {:?}", request);

        // Extract request data
        let day_code = request.into_inner().day_code.to_lowercase();

        // Add error details conditionally
        if day_code.len() != 3 {
            err_details.add_bad_request_violation("day_code", "must consist of three characters");
        }

        if !["mon", "tue", "wed", "thu", "fri", "sat", "sun"].contains(&day_code.as_str()) {
            err_details.add_bad_request_violation("day_code", "code not recognized");
        }

        if err_details.has_bad_request_violations() {
            // Add aditional error details if necessary
            err_details
                .add_help_link("description of link", "https://resource.example.local")
                .set_localized_message("en-US", "message for the user");

            // Generate error status
            let status = Status::with_error_details(
                Code::InvalidArgument,
                "request not recognized",
                err_details,
            )
            .unwrap();

            return Err(status);
        }

        let activity = match day_code.as_str() {
            "sat" | "sun" => "free".into(),
            _ => "work".into(),
        };

        let reply = DayInfoRes { activity };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let my_schedule = MySchedule::default();

    println!("Serving Store at 50051...");

    Server::builder()
        .add_service(ScheduleServer::new(my_schedule))
        .serve(addr)
        .await?;

    Ok(())
}
