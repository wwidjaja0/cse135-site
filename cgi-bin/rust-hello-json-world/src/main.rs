use std::env;
use std::io::{self, Read};
use chrono::{DateTime, Local};
use std::time::SystemTime;

fn main () {
    println!("Cache-Control: no-cache\n");
    println!("Content-Type: application/json\n");

    let content_length = env::var("CONTENT_LENGTH").ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);

    let mut input = String::new();
    io::stdin().take(content_length as u64).read_to_string(&mut input).unwrap();

    let ip = env::var("REMOTE_ADDR").ok().unwrap_or("unknown".to_string());
    let now = SystemTime::now();
    let datetime: DateTime<Local> = DateTime::<Local>::from(now);
    let formatted_datetime = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    println!("<html><body>");
    println!("<h1>Hello World from Rust</h1>");
    println!("This program was generated at: {}\n<br/>", formatted_datetime);
    println!("Your current IP address is: {}<br/>", ip);
    println!("</body></html>");
}
