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

use rocket::fairing::AdHoc;
use hangoutchat::MessageSender;
use gitlab::GitlabTokens;

fn main() {
    rocket::ignite()
            .attach(AdHoc::on_attach(|r| {
                let space = r.config().get_str("hangout_space").unwrap().to_string();
                let token = r.config().get_str("hangout_token").unwrap().to_string();
                let key = r.config().get_str("hangout_key").unwrap().to_string();
                Ok(r.manage(MessageSender {space: space, token: token, key: key}))
            }))
            .attach(AdHoc::on_attach(|r| {
                let gitlab_tokens = r.config().get_str("gitlab_tokens").unwrap().to_string()
                                              .split(",")
                                              .map(|ref x| x.to_string())
                                              .collect();
                Ok(r.manage(GitlabTokens {acceptable_tokens: gitlab_tokens}))
            }))
            .mount("/", routes![router::convert_gitlab_webhook])
            .launch();
}
