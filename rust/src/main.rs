extern crate futures;
extern crate hyper;
extern crate regex;

#[macro_use] extern crate lazy_static;

use futures::future::Future;
use hyper::Method;
use hyper::server::{Http, Request, Response, Service};
use regex::Regex;

struct HelloWorld;

impl Service for HelloWorld {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        lazy_static! {
            static ref GREETING_RE: Regex = Regex::new(r"^/greeting/([a-z]+)$").unwrap();
        }
        let mut response = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                response.set_body("Hello World!");
            },
            _ => {
                let cap = GREETING_RE.captures(req.path()).unwrap();
                response.set_body(format!("Hello, {}", cap.get(1).unwrap().as_str()));
            }
        };
        Box::new(futures::future::ok(response))
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(HelloWorld)).unwrap();
    // let mut server = Server::http("127.0.0.1:3000").unwrap();
    // server.keep_alive(Some(Duration::from_secs(1)));
    // server.keep_alive(None);
    // let _ = server.handle(handler);
    server.run().unwrap();
}
