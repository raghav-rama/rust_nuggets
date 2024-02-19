#[cfg(test)]
mod async_get_request_test {
    use error_chain::error_chain;
    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }
    #[tokio::test]
    async fn test_async_get_request() -> Result<()> {
        let response = reqwest::get("https://httpbin.org/get").await?;
        assert_eq!(response.status().as_u16(), 200);
        Ok(())
    }

    #[tokio::test]
    async fn test_async_get_request_with_error() -> Result<()> {
        let response = reqwest::get("https://httpbin.org/status/404").await?;
        assert_eq!(response.status().as_u16(), 404);
        Ok(())
    }
}
