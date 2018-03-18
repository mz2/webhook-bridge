use rocket_contrib::Json;
use ::gitlab::{WebhookPayload, GitlabTokens};
use ::hangoutchat::MessageSender;
use ::bridge::gitlab_webhook_to_hangout_message;

use rocket::{State, Request};
use rocket::request::{self , FromRequest};
use rocket::http::Status;
use rocket::Outcome::*;

struct ObservedGitlabToken {
    token: String
}

impl<'a, 'r> FromRequest<'a, 'r> for ObservedGitlabToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        match request.headers().get("X-GitLab-Token").next() {
            Some(token) => {
                Success(ObservedGitlabToken { token: token.to_string() })
            },
            None => Failure((Status::BadRequest, ()))
        }
    }
}

type GitlabWebhookForwardingError = ();

#[post("/gitlab/webhook", format = "application/json", data = "<payload>")]
fn convert_gitlab_webhook(payload: Json<WebhookPayload>, 
                          sender: State<MessageSender>, 
                          acceptable_tokens: State<GitlabTokens>, 
                          observed_gitlab_token: ObservedGitlabToken) -> Result<(), Status> {
    match acceptable_tokens.acceptable_tokens.iter().find(|x| **x == observed_gitlab_token.token) {
        Some(_) => {
            sender.send(gitlab_webhook_to_hangout_message(payload.into_inner()));
            Ok(())
        }
        None => Err(Status::BadRequest)
    }
}
