#![allow(non_snake_case)]
#[macro_use]
extern crate nickel;

use nickel::{HttpRouter, Nickel};

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware!("第一个webApp"));
    server.get("/bar", middleware!("this is the /bar handler"));

    server.listen("127.0.0.1:8080").unwrap();
}
