use std::{
    env,
    fs::{self},
    io::{self, Read},
    path::PathBuf,
};
use rand::{distributions::Alphanumeric, Rng};
use urlencoding::decode;

const SESSION_DIR: &str = "/tmp";
const COOKIE_NAME: &str = "CGISESSID";

fn generate_session_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

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

fn read_session_username(session_id: &str) -> Option<String> {
    let path = get_session_file_path(session_id);
    fs::read_to_string(path).ok()
}

fn write_session_username(session_id: &str, username: &str) {
    let path = get_session_file_path(session_id);
    let _ = fs::write(path, username);
}

fn parse_username_from_post() -> Option<String> {
    let content_length = env::var("CONTENT_LENGTH").ok()?.parse::<usize>().ok()?;
    let mut buf = String::new();
    io::stdin().take(content_length as u64).read_to_string(&mut buf).ok()?;
    for pair in buf.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            if k == "username" {
                return Some(decode(v).ok()?.into_owned());
            }
        }
    }
    None
}

fn main() {
    let method = env::var("REQUEST_METHOD").unwrap_or_default();
    let session_id = env::var("HTTP_COOKIE")
        .ok()
        .and_then(|c| get_cookie_value(&c, COOKIE_NAME))
        .unwrap_or_else(generate_session_id);

    let is_new_session = env::var("HTTP_COOKIE")
        .ok()
        .and_then(|c| get_cookie_value(&c, COOKIE_NAME))
        .is_none();

    let mut username = if method.eq_ignore_ascii_case("POST") {
        parse_username_from_post().map(|u| {
            write_session_username(&session_id, &u);
            u
        })
    } else {
        None
    };

    if username.is_none() {
        username = read_session_username(&session_id);
    }

    println!("Cache-Control: no-cache");
    if is_new_session {
        println!("Set-Cookie: {}={}; Path=/", COOKIE_NAME, session_id);
    }
    println!("Content-type: text/html\n");

    println!("<html><head><title>Rust Sessions</title></head><body>");
    println!("<h1>Rust Sessions Page 1</h1>");
    println!("<p style=\"background-color: yellow;\">William Widjaja</p><br/>");

    match username {
        Some(ref name) => println!("<p><b>Name:</b> {}</p>", name),
        None => println!("<p><b>Name:</b> You do not have a name set</p>"),
    }

    println!("<br/><br/>");
    println!("<a href=\"/hw2/rust-cgiform.html\">Rust CGI Form</a><br/>");

    println!("<form style=\"margin-top:30px\" action=\"/cgi-bin/rust-destroy-session.cgi\" method=\"get\">");
    println!("<button type=\"submit\">Destroy Session</button>");
    println!("</form>");

    println!("</body></html>");
}
