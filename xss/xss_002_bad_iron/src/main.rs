extern crate iron;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;

fn handler(req: &mut Request) -> IronResult<Response> {
    let res = req.url.path();
    // Vulnerable to XSS
    let response = format!("<h1>Assessing resource, {:?}!</h1>", res);
    Ok(Response::with((status::Ok, response))) // XSS VULNERABILITY HERE
}
pub fn main() {
    Iron::new(handler).http("localhost:3000").unwrap();
}