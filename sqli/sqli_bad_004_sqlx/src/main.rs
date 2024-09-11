/*
[dependencies]
tokio = { version = "1.39.2", features = ["full"]}
sqlx = { version = "0.8", features = [ "sqlite", "runtime-tokio-rustls" ] }
*/

use std::fs;
use std::io::{BufReader, BufRead, Error, stdin};
use sqlx::{Row, Pool, sqlite::SqlitePool, Sqlite, sqlite::SqliteColumn};

#[tokio::main]
async fn main() {
    let db_key = fs::read_to_string("/var/tmp/tainted_file.txt").unwrap();
    
    let pool_future = SqlitePool::connect("sqlite:///var/tmp/database.db");
    let pool = pool_future.await.unwrap();
    println!("Resolved DB pool");

    let tainted_query = format!("SELECT * FROM someTable WHERE key = '{}'", db_key.trim());

    let result = sqlx::query(&tainted_query) //  Sqli VULNERABILITY HERE
    .fetch_one(&pool)
    .await.unwrap();

    println!("Got results: {:?}", result.columns());
}
