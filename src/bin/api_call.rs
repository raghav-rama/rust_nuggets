use error_chain::error_chain;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Deserialize, Debug)]
struct Users {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let owner = "awslabs";
    let repo = "mountpoint-s3";
    let api_url = format!("https://api.github.com/repos/{}/{}/stargazers", owner, repo);
    let response = reqwest::Client::new()
        .get(&api_url)
        .header(USER_AGENT, "reqwest")
        .send()
        .await?;

    let stargazers = response.json::<Vec<Users>>().await?;
    println!("Stargazers: ");
    for user in stargazers.iter() {
        println!("GitHub Username: {}", user.login);
        println!("ID: {}", user.id);
    }
    println!("Total stargazers: {}", stargazers.len());

    Ok(())
}
