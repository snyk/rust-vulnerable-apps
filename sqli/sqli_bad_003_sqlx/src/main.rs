/*
[dependencies]
sqlx = { version = "0.8", features = [ "sqlite", "runtime-tokio-rustls" ] }
tokio = { version = "1.0", features = ["full"] }
axum = "0.7.5"
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

  let tainted_query = format!("SELECT * FROM someTable WHERE key = '{}'", tainted_key);

  // No edges exist between tainted_key here and db_key in main(), which breaks the taint flow
  let result = sqlx::query(&tainted_query) //  Sqli VULNERABILITY HERE
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
