#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;

extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;
extern crate num_cpus;

use hyper::{Client, Request, Method, Uri};
use hyper_tls::{HttpsConnector};
use hyper::header::{ContentLength, ContentType};
use futures::{Future};
use tokio_core::reactor::Core;

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
    let msg = payload.into_inner();
    match msg.object_kind.as_ref() {
        "issue" => println!("Issue: {}", msg.object_kind),
        _ => println!("Unexpected payload type: ${:?}", msg),
    }

    let space = "AAAA0gIpT4c";
    let key = "AIzaSyDdI0hCZtE6vySjMm-WEfRq3CPzqKqqsHI";
    let token = "6LhzSUAX7eEjfR745yIHyydtoSXhlmD0PGKHlDTfOec%3D";
    let url: Uri = format!("https://chat.googleapis.com/v1/spaces/{}/messages?key={}&token={}", space, key, token).parse().unwrap();

    let outgoing_msg_payload = gitlab_webhook_to_hangout_message(msg);
    let outgoing_msg = serde_json::to_string(&outgoing_msg_payload).unwrap();

    let mut core = Core::new().unwrap();
    let client = Client::configure()
                        .connector(HttpsConnector::new(num_cpus::get(), &core.handle()).unwrap())
                        .build(&core.handle());

    let mut req = Request::new(Method::Post, url);

    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(outgoing_msg.len() as u64));

    println!("Outgoing message: {}", outgoing_msg);
    
    req.set_body(outgoing_msg);

    let post = client.request(req).map(|res| {
        println!("POST: {}", res.status());
    });

    match core.run(post) {
        Ok(response) => println!("Result was alright: {:?}", response),
        Err(e) => {
            println!("Error occurred: {}", e);
        }
    }
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