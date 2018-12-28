extern crate actix_web;
use actix_web::{server, App, HttpRequest};
use std::time::SystemTime;

fn epoch() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }

}
fn index(_req: &HttpRequest) -> String {
    format!("{}", epoch())
}

fn main() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8000")
        .unwrap()
        .run();
}
