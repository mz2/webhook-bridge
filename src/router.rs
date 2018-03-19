use rocket_contrib::Json;
use ::gitlab::{GitLabToken, GitLabTokens, GitLabEventType};
use ::hangoutchat::MessageSender;
use ::bridge::{gitlab_pipeline_event_to_hangout_message, 
              gitlab_issue_to_hangout_message,
              gitlab_merge_request_event_to_hangout_message};

use rocket::{State};
use rocket::http::Status;

use serde_json;

#[post("/gitlab/webhook", format = "application/json", data = "<payload>")]
fn convert_gitlab_webhook(payload: Json, 
                          sender: State<MessageSender>, 
                          acceptable_tokens: State<GitLabTokens>, 
                          gitlab_token: GitLabToken,
                          gitlab_event_type: GitLabEventType) -> Result<(), Status> {
    println!("Incoming '{}' webhook:\n{}\n", gitlab_event_type.event_type, serde_json::to_string_pretty(&payload.clone()).unwrap());
    
    if !["Issue Hook", "Pipeline Hook", "Merge Request Hook"].contains(&gitlab_event_type.event_type.as_str()) {
        return Ok(());
    }

    match acceptable_tokens.acceptable_tokens.iter().find(|x| **x == gitlab_token.token) {
        Some(_) => {
            match gitlab_event_type.event_type.as_str() {
                "Issue Hook" => sender.send(gitlab_issue_to_hangout_message(serde_json::from_value(payload.into_inner()).unwrap())),
                "Pipeline Hook" => sender.send(gitlab_pipeline_event_to_hangout_message(serde_json::from_value(payload.into_inner()).unwrap())),
                "Merge Request Hook" => sender.send(gitlab_merge_request_event_to_hangout_message(serde_json::from_value(payload.into_inner()).unwrap())),
                &_ => { }
            }
            Ok(())
        }
        None => Err(Status::BadRequest)
    }
}
