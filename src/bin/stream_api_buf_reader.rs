use dotenvy::dotenv;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION},
};
use serde::Deserialize;
use serde_json::json;
use std::io::{self, BufRead, BufReader, Write};
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct ChatCompletionChunk {
    choices: Vec<Choices>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Choices {
    index: u64,
    delta: Delta,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Delta {
    content: String,
}

fn main() {
    dotenv().expect(".env file not found");
    let openai_api_key =
        std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not found in .env file");
    let client = Client::new();
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
                {"role": "user", "content": "Hi!"}
            ],
            "response_format": {"type": "json_object"},
            "stream": true
        }
    );

    let response = client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()
        .unwrap();
    let reader = BufReader::new(response);

    let mut buffer = String::new();
    for line in reader.lines() {
        let line = line.unwrap();

        if line.starts_with("data: ") {
            buffer.push_str(&line[6..]); // Extract payload after "data: "
        } else if line.is_empty() {
            // Encountered a complete SSE message
            let chunk_result = serde_json::from_str::<ChatCompletionChunk>(&buffer);
            match chunk_result {
                Ok(chunk) => {
                    print!("{}", chunk.choices[0].delta.content);
                    io::stdout().flush().unwrap();
                }
                Err(_) => {
                    // println!("Error parsing chunk: {:?}", e);
                    continue;
                }
            }
            buffer.clear();
        }
    }
    println!();
}
