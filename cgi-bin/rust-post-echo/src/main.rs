use std::env;
use std::io::{self, Read};
use urlencoding::decode;

fn main() {
    println!("Cache-Control: no-cache");
    println!("Content-type: text/html");
    println!();

    println!("<html><head><title>POST Message Body</title></head>
    <body><h1 align=\"center\">Post Message Body</h1><hr />");
    println!("<p style=\"background-color: yellow;\">William Widjaja</p>");
    println!("<br />");

    // Read the CONTENT_LENGTH environment variable
    let content_length = env::var("CONTENT_LENGTH")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);

    // Read the POST body from stdin
    let mut body = String::new();
    if content_length > 0 {
        io::stdin()
            .take(content_length as u64)
            .read_to_string(&mut body)
            .unwrap_or(0);
    }

    // Show raw body or (null)
    if body.trim().is_empty() {
        println!("<b>Message Body:</b> (null)<br />");
    } else {
        println!("<b>Message Body:</b> {}<br />", body);
    }

    // Decode and parse body like query string
    let params: Vec<(&str, &str)> = body
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            Some((parts.next()?, parts.next().unwrap_or("")))
        })
        .collect();

    if params.is_empty() {
        println!("<p><em>No form parameters were provided.</em></p>");
    } else {
        println!("<ul>");
        for (key, value) in params {
            let key_decoded = decode(key).unwrap_or_default();
            let value_decoded = decode(value).unwrap_or_default();
            println!("<li><strong>{}</strong>: {}</li>", key_decoded, value_decoded);
        }
        println!("</ul>");
    }

    println!("</body></html>");
}
