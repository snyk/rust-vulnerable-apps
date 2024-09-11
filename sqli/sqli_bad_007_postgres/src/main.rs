/*
[dependencies]
postgres = "0.19.8"
*/

use postgres;
use std::fs;

// Based on https://docs.rs/postgres/latest/postgres/struct.Client.html#method.query

fn main() -> Result<(), postgres::Error> {
  let db_key = fs::read_to_string("/var/tmp/tainted_file.txt").unwrap();

  let mut client = postgres::Client::connect("host=localhost user=postgres", postgres::NoTls)?;

  let baz = true;
  let query = format!("SELECT foo FROM bar WHERE baz = {}", db_key);
  //  Sqli VULNERABILITY HERE
  for row in client.query(&query, &[])? {
      let foo: i32 = row.get("foo");
      println!("foo: {}", foo);
  }

  return Ok(())
}
