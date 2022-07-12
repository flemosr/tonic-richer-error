use tonic::{transport::Server, Code, Request, Response, Status};
use tonic_richer_error::{BadRequest, Help, LocalizedMessage, WithErrorDetails};

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
        println!("Got a request: {:?}", request);

        // Extract request data
        let day_code = request.into_inner().day_code.to_lowercase();

        // Create empty BadRequest struct
        let mut bad_request = BadRequest::new(vec![]);

        // Add violations conditionally
        if day_code.len() != 3 {
            bad_request.add_violation("day_code", "must consist of three characters");
        }

        if !["mon", "tue", "wed", "thu", "fri", "sat", "sun"].contains(&day_code.as_str()) {
            bad_request.add_violation("day_code", "code not recognized");
        }

        if !bad_request.is_empty() {
            // Add aditional error details if necessary

            let help = Help::with_link("description of link", "https://resource.example.local");

            let localized_message = LocalizedMessage::new("en-US", "message for the user");

            // Generate error status
            let status = Status::with_error_details_vec(
                Code::InvalidArgument,
                "request not recognized",
                vec![bad_request.into(), help.into(), localized_message.into()],
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
