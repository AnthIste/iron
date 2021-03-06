extern crate iron;

use iron::prelude::*;
use iron::modifiers::Redirect;
use iron::{Url, status};

fn main() {
    let url = Url::parse("http://rust-lang.org").unwrap();

    Iron::new(move |&: _: &mut Request | {
        Ok(Response::with((status::Found, Redirect(url.clone()))))
    }).listen("localhost:3000").unwrap();
    println!("On 3000");
}

