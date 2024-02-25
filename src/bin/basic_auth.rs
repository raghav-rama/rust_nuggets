use error_chain::error_chain;
use reqwest::blocking::Client;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpError(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let client = Client::new();
    let user = String::from("testuser");
    let passwd = Option::<String>::None;
    let url = format!("https://httpbin.org/get");
    let response = client.get(&url).basic_auth(user, passwd).send()?;
    println!("{:?}", response);
    Ok(())
}
