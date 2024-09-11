/*
[dependencies]
sqlx = { version = "0.8", features = [ "sqlite", "runtime-tokio-rustls" ] }
tokio = { version = "1.0", features = ["full"] }
warp = "0.3"
*/

use sqlx::{sqlite::SqlitePool, Pool, Row, Sqlite};
use warp::Filter;

async fn get_pool() -> Pool<Sqlite> {
    let pool_future = SqlitePool::connect("sqlite:///var/tmp/database.db");
    let pool = pool_future.await.unwrap();
    println!("Resolved DB pool");
    return pool;
}

#[tokio::main]
async fn main() {
    // Create a warp filter for the root path
    let route = warp::path::param().then(|tainted_param: String| async move {
        let db = get_pool().await;

        let tainted_query = format!("SELECT * FROM someTable WHERE key = '{}'", tainted_param);

        let result = sqlx::query(&tainted_query) //  Sqli VULNERABILITY HERE
            .fetch_one(&db)
            .await
            .unwrap();

        //  XSS VULNERABILITY HERE
        format!(
            "Got {:?} -> {:?} -> {:?}",
            tainted_param,
            tainted_query,
            result.columns()
        )
    });

    // Start the web server on port 3030
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
