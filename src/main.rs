#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;

//use std::fs::File;
//use std::io::Read;
//use std::string;
use rocket_contrib::Json;

mod gitlab;

#[post("/gitlab/webhook", format = "application/json", data = "<payload>")]
fn hello(payload: Json<gitlab::WebhookPayload>) -> String {
    format!("Hello, {} year old named", payload.object_kind)
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
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