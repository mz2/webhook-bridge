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

mod gitlab;
mod hangoutchat;
mod bridge;
mod router;

fn main() {
    rocket::ignite().mount("/", routes![router::convert_gitlab_webhook]).launch();
}
