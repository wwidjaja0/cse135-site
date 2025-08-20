use std::env;
use std::fs;
use std::path::PathBuf;

const SESSION_DIR: &str = "/tmp";
const COOKIE_NAME: &str = "CGISESSID";

fn get_cookie_value(cookies: &str, name: &str) -> Option<String> {
    for cookie in cookies.split(';') {
        let trimmed = cookie.trim();
        if let Some((k, v)) = trimmed.split_once('=') {
            if k == name {
                return Some(v.to_string());
            }
        }
    }
    None
}

fn get_session_file_path(session_id: &str) -> PathBuf {
    let mut path = PathBuf::from(SESSION_DIR);
    path.push(format!("sess_{}", session_id));
    path
}

fn main() {
    // Try to retrieve session ID from cookie
    let maybe_session_id = env::var("HTTP_COOKIE")
        .ok()
        .and_then(|cookies| get_cookie_value(&cookies, COOKIE_NAME));

    // Attempt to delete session file
    if let Some(session_id) = maybe_session_id {
        let path = get_session_file_path(&session_id);
        let _ = fs::remove_file(path);
    }

    // Output headers (destroy the cookie)
    println!("Cache-Control: no-cache");
    println!("Set-Cookie: CGISESSID=destroyed; Max-Age=0; Path=/");
    println!("Content-type: text/html\n");

    // Output confirmation HTML
    println!("<html><head><title>Rust Session Destroyed</title></head><body>");
    println!("<h1>Session Destroyed</h1>");
    println!("<p style=\"background-color: yellow;\">William Widjaja</p>");
    println!("<p>Your session has been deleted.</p>");
    println!("<a href=\"/cgi-bin/rust-sessions.cgi\">Start New Session</a>");
    println!("</body></html>");
}
