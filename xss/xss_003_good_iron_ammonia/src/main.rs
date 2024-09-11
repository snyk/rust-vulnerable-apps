extern crate iron;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;

use ammonia::clean;

fn handler(req: &mut Request) -> IronResult<Response> {
    let res = req.url.path();
    // Vulnerable to XSS
    let response = format!("<h1>Assessing resource, {:?}!</h1>", res);

    // Sanitized by clean
    let sanitized = clean(&*response);
    Ok(Response::with((status::Ok, sanitized))) // NOT VULNERABLE TO XSS HERE
}
pub fn main() {
    Iron::new(handler).http("localhost:3000").unwrap();
}