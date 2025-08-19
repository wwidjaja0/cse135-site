use std::env;

fn main() {
    println!("Cache-Control: no-cache");
    println!("Content-type: text/html");
    println!();
    println!("<html><head><title>Environment Variables</title></head><body><h1 align=\"center\">Environment Variables</h1><hr>");

    println!("<p style=\"background-color: yellow;\">William Widjaja</p>");
    println!("<br />");

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    println!("</body></html>");
}
