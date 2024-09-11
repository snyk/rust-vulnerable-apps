/*
[dependencies]
tokio = { version = "1.39.2", features = ["full"]}
sqlx = { version = "0.8", features = [ "sqlite", "runtime-tokio-rustls" ] }
*/

use std::fs;
use std::io::{BufReader, BufRead, Error, stdin};
use sqlx::{Row, Pool, sqlite::SqlitePool, Sqlite, sqlite::SqliteColumn};

async fn get_pool() -> Pool<Sqlite> {
  let pool_future = SqlitePool::connect("sqlite:///var/tmp/database.db");
  let pool = pool_future.await.unwrap();
  println!("Resolved DB pool");
  return pool;
}

async fn vuln_func(tainted_key: String, db: &Pool<Sqlite>) -> sqlx::sqlite::SqliteRow {

  // Prepared statement sanitizer
  let result = sqlx::query("SELECT * FROM someTable WHERE key = $1")// NOT VULNERABLE TO Sqli HERE
  .bind(tainted_key.trim())
  .fetch_one(db)
  .await.unwrap();

  return result;
}

#[tokio::main]
async fn main() {
    let db_key = fs::read_to_string("/var/tmp/tainted_file.txt").unwrap();
    
    let db = get_pool().await;

    let query_results = vuln_func(db_key, &db).await; 

    println!("Got results: {:?}", query_results.columns());
}
