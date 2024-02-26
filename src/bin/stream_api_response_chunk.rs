use dotenvy::dotenv;
use error_chain::error_chain;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION},
    Client,
};
use serde_json::json;
use std::env;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect(".env file not found");
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in .env file");
    let url = "https://api.openai.com/v1/chat/completions";
    let mut headers = HeaderMap::new();

    headers.insert(
        ACCEPT,
        HeaderValue::from_bytes(b"text/event-stream").unwrap(),
    );
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", openai_api_key)).unwrap(),
    );

    let request_body = json!(
        {
            "model": "gpt-4-turbo-preview",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful assistant, who generates output in JSON format only."
                },
                {"role": "user", "content": "What is the meaning of life?"}
            ],
            "response_format": {"type": "json_object"},
            "stream": true
        }
    );

    let client = Client::new();
    let mut response = client
        .post(url)
        .headers(headers.clone())
        .json(&request_body)
        .send()
        .await?;

    while let Some(chunk) = response.chunk().await? {
        println!("Chunk: {:?}", chunk);
    }

    Ok(())
}
