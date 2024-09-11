// ~ https://github.com/seanmonstar/reqwest/blob/88bd9be6/examples/form.rs#L10
// Example inspired from reqwest examples, MIT Licensed

// Combined with warp for taint

/*
tokio = { version = "1.0", features = ["full"] }
warp = "0.3"
reqwest = { version = "0.12", features = ["json"] }
 */

 use warp::Filter;


 #[tokio::main]
 async fn main() {
     let routes = warp::path::param().then(|tainted_param: String| async move {
         let response = reqwest::Client::new()
             .post(format!("https://{}", tainted_param)) //  Ssrf VULNERABILITY HERE
             .form(&[("one", "1")])
             .send()
             .await
             .expect("send");
 
         format!("{} {}", tainted_param, response.status()) //  XSS VULNERABILITY HERE
     });
 
     warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
 }
 