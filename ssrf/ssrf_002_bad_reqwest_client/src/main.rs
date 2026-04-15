
 use std::convert::Infallible;
use std::time::Duration;
use warp::{http::StatusCode, reply::with_status, Filter};

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("failed to build client");

    let status_route = {
        let client = client.clone();
        warp::get()
            .and(warp::path("status"))
            .and(warp::path::end())
            .and_then(move || {
                let client = client.clone();
                async move { proxy_upstream(client, "/status").await }
            })
    };

    let profile_route = {
        let client = client.clone();
        warp::get()
            .and(warp::path("profile"))
            .and(warp::path::end())
            .and_then(move || {
                let client = client.clone();
                async move { proxy_upstream(client, "/profile").await }
            })
    };

    let routes = status_route.or(profile_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn proxy_upstream(
    client: reqwest::Client,
    upstream_path: &'static str,
) -> Result<impl warp::Reply, Infallible> {
    let url = format!("https://example.com{}", upstream_path);

    let reply = match client.get(&url).send().await {
        Ok(response) => {
            let status = StatusCode::from_u16(response.status().as_u16())
                .unwrap_or(StatusCode::BAD_GATEWAY);

            with_status(
                format!("upstream responded with {}", response.status()),
                status,
            )
        }
        Err(_) => with_status(
            "upstream request failed".to_string(),
            StatusCode::BAD_GATEWAY,
        ),
    };

    Ok(reply)
}
