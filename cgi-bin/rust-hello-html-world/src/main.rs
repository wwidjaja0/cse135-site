use std::env;
use std::io::{self, Read};
use chrono::{DateTime, Local};
use std::time::SystemTime;

fn main () {
    println!("Cache-Control: no-cache");
    println!("Content-type: text/html");
    println!();

    let content_length = env::var("CONTENT_LENGTH").ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);

    let mut input = String::new();
    io::stdin().take(content_length as u64).read_to_string(&mut input).unwrap();

    let now = SystemTime::now();
    let datetime: DateTime<Local> = DateTime::<Local>::from(now);
    let formatted_datetime = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    let ip = env::var("REMOTE_ADDR").ok().unwrap_or("unknown".to_string());

    println!("<html><body>");
    println!("<h1>Hello, Rust!</h1>");
    println!("<p style=\"background-color: yellow;\">William Widjaja</p>");
    println!("<br />");
    println!("<p>This page was generated with the Rust programming langauge</p>");
    println!("Current Time: {}", formatted_datetime);
    println!("Your IP Address: {}<br/>", ip);
    println!("</body></html>");
}
