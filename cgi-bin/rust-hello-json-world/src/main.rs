use std::env;
use chrono::{DateTime, Local};
use std::time::SystemTime;
use serde_json::json;

fn main () {
    println!("Cache-Control: no-cache");
    println!("Content-type: text/html");
    println!();

    let ip = env::var("REMOTE_ADDR").ok().unwrap_or("unknown".to_string());
    let now = SystemTime::now();
    let datetime: DateTime<Local> = DateTime::<Local>::from(now);
    let formatted_datetime = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    let message = json!({"title":"Hello, Rust!", "heading":"Hello, Rust!", "author":"William Widjaja", "message":"This page was generated with the Rust programming language", "time":formatted_datetime, "IP":ip});

    println!("{}", serde_json::to_string_pretty(&message).unwrap())
}
