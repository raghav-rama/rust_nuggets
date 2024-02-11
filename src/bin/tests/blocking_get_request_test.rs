#[cfg(test)]
mod blocking_get_request_test {
    use error_chain::error_chain;
    use reqwest::blocking::get;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    struct Headers<'a> {
        date: &'a str,
        #[serde(rename = "content-type")]
        content_type: &'a str,
        #[serde(rename = "content-length")]
        content_length: &'a str,
        connection: &'a str,
        server: &'a str,
        #[serde(rename = "access-control-allow-origin")]
        access_control_allow_origin: &'a str,
        #[serde(rename = "access-control-allow-credentials")]
        access_control_allow_credentials: &'a str,
    }

    error_chain!(
        foreign_links {
            HttpRequest(reqwest::Error);
            Io(std::io::Error);
        }
    );

    #[test]
    fn it_deserializes_properly() -> Result<()> {
        let res = get("http://localhost/get")?;
        let headers = res.headers();
        let mut headers_hashmap = HashMap::new();
        headers.iter().for_each(|(name, value)| {
            headers_hashmap.insert(name.as_str(), value.to_str().unwrap());
        });
        let raw_headers_json = json!(headers_hashmap);
        let headers_json_str = serde_json::to_string(&raw_headers_json);
        let binding = headers_json_str.unwrap();
        let headers_json_deserialized = serde_json::from_str::<Headers>(binding.as_str());
        match headers_json_deserialized {
            Ok(ref result) => {
                assert_eq!(result.server.contains("gunicorn"), true);
                assert_eq!(result.content_type.contains("application/json"), true);
                assert_eq!(result.access_control_allow_credentials, "true");
                assert_eq!(result.access_control_allow_origin, "*");
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
        Ok(())
    }

    #[test]
    fn it_gets_a_response() -> Result<()> {
        let response = get("http://localhost/get")?;
        assert_eq!(response.status(), 200);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn it_panics_on_invalid_url() {
        let _ = get("http://localhost:12345").unwrap();
    }

    #[test]
    fn it_handles_invalid_url() {
        let response = get("http://localhost:12345");
        assert!(response.is_err());
    }

    #[test]
    #[should_panic]
    fn it_handles_invalid_url_with_match() {
        let response = get("http://localhost:12345");
        match response {
            Ok(_) => panic!("This should not be Ok!"),
            Err(e) => panic!("{}", e),
        }
    }
}
