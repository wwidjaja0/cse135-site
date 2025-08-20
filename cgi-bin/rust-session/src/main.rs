use std::env;
use std::io::{self, Read};
use chrono::{DateTime, Local};
use std::time::SystemTime;

fn parse_username_from_body(body: &str) -> Option<String> {
    // Expecting application/x-www-form-urlencoded from the form
    for pair in body.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            if key == "username" {
                let decoded = urlencoding::decode(value).ok()?.into_owned();
                return Some(decoded);
            }
        }
    }
    None
}

fn main() {
    // Read request method and content length for POST
    let method = env::var("REQUEST_METHOD").unwrap_or_default();
    let mut name_from_post: Option<String> = None;

    if method.eq_ignore_ascii_case("POST") {
        if let Ok(cl_str) = env::var("CONTENT_LENGTH") {
            if let Ok(cl) = cl_str.parse::<usize>() {
                let mut buf = String::with_capacity(cl);
                io::stdin().take(cl as u64).read_to_string(&mut buf).ok();
                name_from_post = parse_username_from_body(&buf);
            }
        }
    }

    // Determine cookie to set and what to show
    let mut cookie_to_set: Option<String> = None;
    let mut cookie_value: Option<String> = None;

    if let Some(n) = name_from_post {
        // Set cookie to the provided name
        cookie_to_set = Some(n.clone());
        cookie_value = Some(n);
    } else if let Ok(cookie) = env::var("HTTP_COOKIE") {
        if cookie != "destroyed" && !cookie.is_empty() {
            cookie_value = Some(cookie);
        }
    }

    // Headers
    println!("Cache-Control: no-cache");
    if let Some(val) = cookie_to_set.as_deref() {
        println!("Set-Cookie: {}", val);
    }
    println!("Content-type: text/html");
    println!();

    // Body
    let now = SystemTime::now();
    let datetime: DateTime<Local> = DateTime::<Local>::from(now);
    let _formatted_datetime = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    println!("<html>");
    println!("<head><title>Rust Sessions</title></head>");
    println!("<body>");
    println!("<h1>Rust Sessions</h1>");
    println!("<p style=\"background-color: yellow;\">William Widjaja</p>");
    println!("<br />");

    println!("<table>");
    if let Some(cv) = cookie_value.as_deref() {
        println!("<tr><td>Cookie:</td><td>{}</td></tr>", cv);
    } else {
        println!("<tr><td>Cookie:</td><td>None</td></tr>");
    }
    println!("</table>");

    println!("<br/>");
    println!("<a href=\"/hw2/rust-cgiform.html\">Rust CGI Form</a><br/>");

    println!("<form style=\"margin-top:30px\" action=\"/cgi-bin/rust-destroy-session.cgi\" method=\"get\">");
    println!("<button type=\"submit\">Destroy Session</button>");
    println!("</form>");

    println!("</body></html>");
}
