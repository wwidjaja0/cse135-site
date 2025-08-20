fn main() {
    // Headers
    println!("Cache-Control: no-cache");
    println!("Set-Cookie: destroyed");
    println!("Content-type: text/html");
    println!();

    // Body
    println!("<html>");
    println!("<head><title>Rust Session Destroyed</title></head>");
    println!("<body>");
    println!("<h1>Rust Session Destroyed</h1>");

    println!("<a href=\"/cgi-bin/rust-session.cgi\">Back to Page 1</a><br/>");
    println!("<a href=\"/hw2/rust-cgiform.html\">Rust CGI Form</a><br/>");
    println!("<p style=\"background-color: yellow;\">William Widjaja</p>");

    println!("</body></html>");
}
