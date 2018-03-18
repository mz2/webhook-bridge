#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;

extern crate hyper;
extern crate futures;
extern crate tokio_core;

use hyper::{Request, Method, Client, Uri};
use hyper::header::{ContentLength, ContentType};
use futures::{Future, Stream};
use tokio_core::reactor::Core;

//use std::fs::File;
//use std::io::Read;
//use std::string;
use rocket_contrib::Json;

mod gitlab;
mod hangoutchat;

#[post("/gitlab/webhook", format = "application/json", data = "<payload>")]
fn convert_gitlab_webhook(payload: Json<gitlab::WebhookPayload>) -> Json<hangoutchat::TextMessagePayload> {
    let msg = payload.into_inner();
    match msg.object_kind.as_ref() {
        "issue" => println!("Issue: {}", msg.object_kind),
        _ => println!("Unexpected payload type: ${:?}", msg),
    }

    let space = "AAAA0gIpT4c";
    let key = "AIzaSyDdI0hCZtE6vySjMm-WEfRq3CPzqKqqsHI";
    let token = "6LhzSUAX7eEjfR745yIHyydtoSXhlmD0PGKHlDTfOec%3D";
    let url: Uri = format!("https://chat.googleapis.com/v1/spaces/${}/messages?key={}&token={}", space, key, token).parse().unwrap();

    let outgoing_msg_payload = hangoutchat::TextMessagePayload { text: "foo".to_string() };
    let outgoing_msg = serde_json::to_string(&outgoing_msg_payload).unwrap();

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let mut req = Request::new(Method::Post, url);

    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(outgoing_msg.len() as u64));
    req.set_body(outgoing_msg);

    let post = client.request(req).map(|res| {
        println!("POST: {}", res.status());
    });

    core.run(post).unwrap();
    
    return Json(outgoing_msg_payload);
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