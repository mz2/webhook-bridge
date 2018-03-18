#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;

extern crate futures;
extern crate tokio_core;
extern crate num_cpus;
extern crate hyper;
extern crate hyper_tls;
use rocket_contrib::Json;

mod gitlab;
mod hangoutchat;

fn gitlab_webhook_to_hangout_message(payload: gitlab::WebhookPayload) -> hangoutchat::TextMessage {
    let prev_assignees: Vec<String> = payload.changes.assignees.previous.iter().map({ |ref x| format!("@{}", x.username) }).collect();
    let current_assignees: Vec<String> = payload.changes.assignees.current.iter().map({ |ref x| format!("@{}", x.username) }).collect();
    
    let txt = match (prev_assignees, current_assignees) {
        (ref prev, ref current) if prev != current => { 
            let current_assignees_str = if current.len() > 0 { current.join(",") } else { "no-one".to_string() };
            format!("Assignee of {} #{} ('{}') changed to {:?}.", payload.project.name, payload.object_attributes.iid, payload.object_attributes.title, current_assignees_str)
        },
        _ => format!("Unexpected message payload: {:?}", payload)
    };
    
    let msg = hangoutchat::TextMessage { text: txt };
    return msg;
}

#[post("/gitlab/webhook", format = "application/json", data = "<payload>")]
fn convert_gitlab_webhook(payload: Json<gitlab::WebhookPayload>) {
    let sender = hangoutchat::Sender {
        space: "AAAA0gIpT4c".to_string(),
        key: "AIzaSyDdI0hCZtE6vySjMm-WEfRq3CPzqKqqsHI".to_string(),
        token: "6LhzSUAX7eEjfR745yIHyydtoSXhlmD0PGKHlDTfOec%3D".to_string()
    };

    sender.send(gitlab_webhook_to_hangout_message(payload.into_inner()));
}

fn main() {
    rocket::ignite().mount("/", routes![convert_gitlab_webhook]).launch();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_transformation() {
        let mut file = File::open("./src/fixtures/gitlab-issue-event.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let json = serde_json::from_str(data);

        /*
        json.
        let object_kind = json.find_path(&["object_kind"]).unwrap();

        if (object_kind == "issue") {
            //println!("foo {}", );
        }
        */
    }
}