use iron::headers::Host;
use iron::prelude::*;
use iron::status;
use iron::Request;

pub fn main() {
    Iron::new(|req: &mut Request| {
        let headers = &req.headers;
        let extracted = headers.get::<Host>();

        let mut payload = String::new();
        let query = req.url.query()
            .expect("some data")
            .to_owned();

        let request_url = req.url.to_string();

        let data = reqwest::blocking::get(request_url) //  Ssrf VULNERABILITY HERE
            .expect("failed")
            .text()
            .expect("error");

        let response = data.to_owned();
        let mut r = Response::with((status::Ok, response.to_owned())); //  XSS VULNERABILITY HERE
        Ok(r)
    })
        .http("localhost:8080")
        .unwrap();
}
