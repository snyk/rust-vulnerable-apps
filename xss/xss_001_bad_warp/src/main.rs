use warp::Filter;

#[tokio::main]
async fn main() {
    // Create a warp filter for the root path
    let hello = warp::path::param()
        .map(|name: String| {
            // Vulnerable to XSS because it directly includes user input in the response
            format!("<h1>Hello, {}!</h1>", name) // XSS VULNERABILITY HERE
        });

    // Start the web server on port 3030
    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}