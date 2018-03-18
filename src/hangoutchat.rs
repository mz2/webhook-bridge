#[derive(Serialize, Deserialize, Debug)]
pub struct TextMessage {
    pub text: String 
}

use hyper::{Client, Request, Method, Uri};
use hyper_tls::{HttpsConnector};
use hyper::header::{ContentLength, ContentType};
use futures::{Future};
use tokio_core::reactor::Core;

use serde_json;
use num_cpus;

pub struct Sender {
    pub space: String,
    pub key: String,
    pub token: String
}

impl Sender {
    fn url(&self) -> Uri {
        return format!("https://chat.googleapis.com/v1/spaces/{}/messages?key={}&token={}", self.space, self.key, self.token).parse().unwrap();
    }
    
    pub fn send(&self, outgoing_msg_payload: TextMessage) {
        let outgoing_msg = serde_json::to_string(&outgoing_msg_payload).unwrap();
        let mut req = Request::new(Method::Post, self.url());

        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(outgoing_msg.len() as u64));

        println!("Outgoing message: {}", outgoing_msg);
        
        req.set_body(outgoing_msg);

        let mut core = Core::new().unwrap();
        let client = Client::configure()
                                    .connector(HttpsConnector::new(num_cpus::get(), &core.handle()).unwrap())
                                    .build(&core.handle());

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
}