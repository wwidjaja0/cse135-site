use std::env;
use std::io::{self, Read};

fn main () {
    println!("Content-Type: text/html\n");

    let content_length = env::var("CONTENT_LENGTH").ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);

    let mut input = String::new();
    io::stdin().take(content_length as u64).read_to_string(&mut input).unwrap();

    println!("<html><body>");
    println!("<h1>Env!</h1>");
    if !input.is_empty() {
        println!("<p>Received POST data: {}</p>", input);
    }
    println!("</body></html>");
}
