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

        let request_url = req.url.to_owned();

        let path = format!("http://my.app/{request_url}");

        let data = reqwest::blocking::get(path) // NOT VULNERABLE TO Ssrf HERE
            .expect("failed")
            .text()
            .expect("error");

        let response = data.to_owned();
        let mut r = Response::with((status::Ok, response.to_owned()));
        Ok(r)
    })
        .http("localhost:8080")
        .unwrap();
}
