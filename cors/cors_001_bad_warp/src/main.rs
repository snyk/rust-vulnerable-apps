use warp::Filter;

#[tokio::main]
async fn main() {
    // Setting up secure CORS, allowing requests only from a specific origin, with GET and POST methods and specific headers
    let cors = warp::cors()
        .allow_any_origin() //  TooPermissiveCors VULNERABILITY HERE
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["Content-Type", "Authorization"]);

    // Define the route
    let route = warp::path("example")
        .and(warp::get())
        .map(|| "Hello, CORS!")
        .with(cors);

    // Start the server
    warp::serve(route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}