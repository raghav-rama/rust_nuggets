#[cfg(test)]
mod blocking_get_request_test {
    use error_chain::error_chain;
    use reqwest::blocking::get;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::collections::HashMap;
    use std::env;

    fn get_url() -> &'static str {
        let current_env = get_current_environment();
        if current_env == Env::Local("true") {
            println!("Setting localhost");
            "http://localhost"
        } else if current_env == Env::GithubActions("true") {
            println!("Setting Github Actions");
            "http://172.17.0.2"
        } else {
            println!("Setting undefined");
            "undefined"
        }
    }

    fn get_port() -> &'static str {
        let current_env = get_current_environment();
        if current_env == Env::Local("true") {
            println!("Setting port 8000");
            "8080"
        } else if current_env == Env::GithubActions("true") {
            println!("Setting port 80");
            "80"
        } else {
            println!("Setting port undefined");
            "undefined"
        }
    }

    fn get_route() -> &'static str {
        return "/get";
    }

    #[derive(Debug, PartialEq, Eq)]
    enum Env<'a> {
        Local(&'a str),
        GithubActions(&'a str),
        Undefined,
    }

    fn get_current_environment() -> Env<'static> {
        match env::var("LOCAL") {
            Ok(val) => {
                if val == "true" {
                    println!("Local environment detected");
                    return Env::Local("true");
                } else {
                    Env::Local("false")
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Env::Undefined;
            }
        };
        match env::var("GITHUB_ACTIONS") {
            Ok(val) => {
                if val == "true" {
                    println!("Github Actions environment detected");
                    return Env::GithubActions("true");
                } else {
                    Env::GithubActions("false")
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return Env::Undefined;
            }
        }
    }

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
        let url = get_url();
        let port = get_port();
        let route = get_route();
        let res = if port == "80" {
            get(format!("{}{}", url, route))?
        } else {
            get(format!("{}:{}{}", url, port, route))?
        };
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
        let url = get_url();
        let port = get_port();
        let route = get_route();
        let res = if port == "80" {
            get(format!("{}{}", url, route))?
        } else {
            get(format!("{}:{}{}", url, port, route))?
        };
        assert_eq!(res.status(), 200);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn it_panics_on_invalid_url() {
        let _ = get("http://172.17.0.2:12345").unwrap();
    }

    #[test]
    fn it_handles_invalid_url() {
        let response = get("http://172.17.0.2:12345");
        assert!(response.is_err());
    }

    #[test]
    #[should_panic]
    fn it_handles_invalid_url_with_match() {
        let response = get("http://172.17.0.2:12345");
        match response {
            Ok(_) => panic!("This should not be Ok!"),
            Err(e) => panic!("{}", e),
        }
    }
}
