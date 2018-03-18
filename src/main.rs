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
mod bridge;
mod router;

fn main() {
    rocket::ignite().mount("/", routes![router::convert_gitlab_webhook]).launch();
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