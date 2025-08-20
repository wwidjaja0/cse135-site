use std::{
    env,
    fs::{self},
    io::{self, Read},
    path::PathBuf,
};
use rand::{distributions::Alphanumeric, Rng};
use urlencoding::decode;
use cookie::Cookie;

const SESSION_DIR: &str = "/tmp";
const COOKIE_NAME: &str = "CGISESSID";

fn generate_session_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

fn get_cookie_value(header: &str, name: &str) -> Option<String> {
    // The cookie crate parses individual cookie strings; split on ';' and try each
    for kv in header.split(';') {
        let s = kv.trim();
        if s.is_empty() { continue; }
        if let Ok(parsed) = Cookie::parse(s) {
            if parsed.name() == name {
                return Some(parsed.value().to_string());
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

fn session_exists(session_id: &str) -> bool {
    get_session_file_path(session_id).exists()
}

fn read_session_username(session_id: &str) -> Option<String> {
    let path = get_session_file_path(session_id);
    fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn write_session_username(session_id: &str, username: &str) {
    let path = get_session_file_path(session_id);
    let _ = fs::write(path, username);
}

fn touch_session(session_id: &str) {
    // Ensure a session file exists even if no username is set yet
    let path = get_session_file_path(session_id);
    if !path.exists() {
        let _ = fs::write(path, "");
    }
}

fn parse_username_from_post() -> Option<String> {
    let content_length = env::var("CONTENT_LENGTH").ok()?.parse::<usize>().ok()?;
    let mut buf = String::new();
    io::stdin().take(content_length as u64).read_to_string(&mut buf).ok()?;
    for pair in buf.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            if k == "username" {
                let decoded = decode(v).ok()?.into_owned();
                let trimmed = decoded.trim().to_string();
                if trimmed.is_empty() {
                    return None;
                }
                return Some(trimmed);
            }
        }
    }
    None
}

fn main() {
    let method = env::var("REQUEST_METHOD").unwrap_or_default();

    // Determine session id: if cookie present and looks valid, use it; otherwise generate new
    let raw_cookie = env::var("HTTP_COOKIE").ok();
    let cookie_id = raw_cookie
        .as_deref()
        .and_then(|c| get_cookie_value(c, COOKIE_NAME))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && s != "destroyed");

    let session_id = match cookie_id {
        Some(id) => {
            // Ensure a session file exists for existing cookie-based sessions
            touch_session(&id);
            id
        }
        None => {
            let id = generate_session_id();
            // Create an empty session file so subsequent reloads find it
            touch_session(&id);
            id
        }
    };

    let mut username = if method.eq_ignore_ascii_case("POST") {
        // Only update the stored username if a non-empty value was provided
        if let Some(u) = parse_username_from_post() {
            write_session_username(&session_id, &u);
            Some(u)
        } else {
            None
        }
    } else {
        None
    };

    if username.is_none() {
        username = read_session_username(&session_id);
    }

    println!("Cache-Control: no-cache");
    // Use cookie crate to build Set-Cookie header
    let c = Cookie::build(Cookie::new(COOKIE_NAME, session_id.clone()))
        .path("/")
        .build();
    println!("Set-Cookie: {}", c.to_string());
    println!("Content-type: text/html\n");

    println!("<html><head><title>Rust Sessions</title></head><body>");
    println!("<h1>Rust Sessions Page 1</h1>");
    println!("<p style=\"background-color: yellow;\">William Widjaja</p><br/>");

    // Display the cookie (session id) value used for this request
    println!("<p><b>Cookie:</b> {}={}</p>", COOKIE_NAME, session_id);

    match username {
        Some(ref name) if !name.trim().is_empty() => println!("<p><b>Name:</b> {}</p>", name),
        _ => println!("<p><b>Name:</b> You do not have a name set</p>"),
    }

    println!("<br/><br/>");
    println!("<a href=\"/hw2/rust-cgiform.html\">Rust CGI Form</a><br/>");

    println!("<form style=\"margin-top:30px\" action=\"/cgi-bin/rust-destroy-session.cgi\" method=\"get\">");
    println!("<button type=\"submit\">Destroy Session</button>");
    println!("</form>");

    println!("</body></html>");
}
