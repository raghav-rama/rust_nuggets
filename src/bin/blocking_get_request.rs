use error_chain::error_chain;
use reqwest::blocking::get;
use std::io::Read;

error_chain! {
    foreign_links {
        HttpRequest(reqwest::Error);
        Io(std::io::Error);
    }
}

fn main() -> Result<()> {
    let mut response = get("http://localhost/get")?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;
    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());
    println!("Body: {}", body);
    Ok(())
}
