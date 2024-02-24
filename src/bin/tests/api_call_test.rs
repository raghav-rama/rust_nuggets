#[cfg(test)]
mod api_call_test {
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
    #[allow(dead_code)]
    struct Users {
        login: String,
        id: u32,
    }
    #[tokio::test]
    async fn test_api_call() -> Result<()> {
        let owner = "awslabs";
        let repo = "mountpoint-s3";
        let api_url = format!("https://api.github.com/repos/{}/{}/stargazers", owner, repo);
        let response = reqwest::Client::new()
            .get(&api_url)
            .header(USER_AGENT, "reqwest")
            .send()
            .await?;

        let stargazers = response.json::<Vec<Users>>().await?;
        assert_eq!(stargazers.len(), 30);
        Ok(())
    }
}
