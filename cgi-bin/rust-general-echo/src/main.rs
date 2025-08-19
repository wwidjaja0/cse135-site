use std::env;
use std::io::{self, Read};

fn main() {
    println!("Cache-Control: no-cache");
    println!("Content-type: text/html\n");

    println!(
        "<html><head><title>General Request Echo</title></head>
        <body><h1 align=\"center\">General Request Echo</h1><hr/>"
    );

    println!("<p style=\"background-color: yellow;\">William Widjaja</p>");
    println!("<br />");

    println!("<table>");

    let protocol = env::var("SERVER_PROTOCOL").unwrap_or_else(|_| "(unknown)".to_string());
    let method = env::var("REQUEST_METHOD").unwrap_or_else(|_| "(unknown)".to_string());

    println!("<tr><td>Protocol:</td><td>{}</td></tr>", protocol);
    println!("<tr><td>Method:</td><td>{}</td></tr>", method);

    // Read from stdin (up to 1000 bytes like in C)
    let mut buffer = [0; 1000];
    let bytes_read = io::stdin().read(&mut buffer).unwrap_or(0);
    let message_body = if bytes_read > 0 {
        String::from_utf8_lossy(&buffer[..bytes_read]).to_string()
    } else {
        "(null)".to_string()
    };

    println!("<tr><td>Message Body:</td><td>{}</td></tr>", message_body);
    println!("</table></body></html>");
}
