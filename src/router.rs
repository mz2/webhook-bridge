use rocket_contrib::Json;
use ::gitlab::{WebhookPayload, GitLabToken, GitLabTokens, GitLabEventType};
use ::hangoutchat::MessageSender;
use ::bridge::gitlab_webhook_to_hangout_message;

use rocket::{State};
use rocket::http::Status;

#[post("/gitlab/webhook", format = "application/json", data = "<payload>")]
fn convert_gitlab_webhook(payload: Json<WebhookPayload>, 
                          sender: State<MessageSender>, 
                          acceptable_tokens: State<GitLabTokens>, 
                          gitlab_token: GitLabToken,
                          gitlab_event_type: GitLabEventType) -> Result<(), Status> {
    if !["Issue Hook", "Pipeline Hook"].contains(&gitlab_event_type.event_type.as_str()) {
        return Ok(());
    }

    match acceptable_tokens.acceptable_tokens.iter().find(|x| **x == gitlab_token.token) {
        Some(_) => {
            sender.send(gitlab_webhook_to_hangout_message(payload.into_inner()));
            Ok(())
        }
        None => Err(Status::BadRequest)
    }
}
