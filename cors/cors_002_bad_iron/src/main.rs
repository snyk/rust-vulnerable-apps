/*
https://cwe.mitre.org/data/definitions/942.html
*/
use iron::headers::AccessControlAllowOrigin;
use iron::prelude::*;
use iron::status;

#[allow(dead_code)]
pub fn main() {
    Iron::new(|_: &mut Request| {
        let mut r = Response::with((status::Ok, "Hello World!"));
        // Respond with a malicious CORS header allowing all origins
        r.headers.set(AccessControlAllowOrigin::Any); //  TooPermissiveCors VULNERABILITY HERE
        Ok(r)
    })
    .http("localhost:8080")
    .unwrap();
}
