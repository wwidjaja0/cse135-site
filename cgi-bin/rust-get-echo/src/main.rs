use std::collections::HashMap;
use std::env;
use urlencoding::decode;

fn main() {
    println!("Cache-Control: no-cache");
    println!("Content-type: text/html");
    println!();

    println!("<html><head><title>GET Request Echo</title></head>
    <body><h1 align=\"center\">Get Request Echo</h1><hr />");
    println!("<p style=\"background-color: yellow;\">William Widjaja</p>");
    println!("<br />");

    // Get the query string from the environment
    let query = env::var("QUERY_STRING").unwrap_or_default();
    println!("<b>Query String:</b> {}<br />\n", &query);

    // Parse the query string into key-value pairs
    let params: HashMap<_, _> = query
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?;
            let value = parts.next().unwrap_or("");
            Some((decode(key).ok()?, decode(value).ok()?))
        })
        .collect();

    if params.is_empty() {
        println!("<p><em>No query parameters were provided.</em></p>");
    } else {
        println!("<ul>");
        for (key, value) in &params {
            println!("<li><strong>{}</strong>: {}</li>", key, value);
        }
        println!("</ul>");
    }

    println!("</body></html>");
}
