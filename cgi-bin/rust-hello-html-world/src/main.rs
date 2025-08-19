use std::env;
use chrono::{DateTime, Local};
use std::time::SystemTime;

fn main () {
    println!("Cache-Control: no-cache");
    println!("Content-type: text/html");
    println!();

    let now = SystemTime::now();
    let datetime: DateTime<Local> = DateTime::<Local>::from(now);
    let formatted_datetime = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    let ip = env::var("REMOTE_ADDR").ok().unwrap_or("unknown".to_string());

    println!("<html><body>");
    println!("<h1>Hello, Rust!</h1>");
    println!("<p style=\"background-color: yellow;\">William Widjaja</p>");
    println!("<br />");
    println!("<p>This page was generated with the Rust programming langauge</p>");
    println!("Current Time: {}<br/>", formatted_datetime);
    println!("Your IP Address: {}", ip);
    println!("</body></html>");
}
