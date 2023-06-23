#![allow(non_snake_case)]
#![allow(unreachable_code)]
#[macro_use]
extern crate nickel;

use nickel::{HttpRouter, Nickel};
use std::collections::HashMap;

fn main() {
    let mut server = Nickel::new();

    server.get(
        "/",
        middleware! { |_, response|
            let mut data = HashMap::new();
            data.insert("name", "user");//传入值
            return response.render("examples/assets/template.tpl", &data);
        },
    );

    server.listen("127.0.0.1:6767").unwrap();
}
