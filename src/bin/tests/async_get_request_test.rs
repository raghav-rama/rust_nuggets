#[cfg(test)]
mod async_get_request_test {
    use error_chain::error_chain;
    use std::env;
    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    enum Environment {
        GithubActions,
        Production,
        Development,
        Unknown,
    }
    #[derive(Debug)]
    struct EnvironmentType {
        env: Environment,
        url: String,
        port: String,
        route: String,
    }

    impl EnvironmentType {
        fn new() -> Self {
            EnvironmentType {
                env: Environment::Unknown,
                url: "https://httpbin.org".to_string(),
                port: "443".to_string(),
                route: "/get".to_string(),
            }
        }
        fn get_env(&self) -> Environment {
            let env: Environment;
            if self.is_local() {
                env = Environment::Development;
            } else if self.is_production() {
                env = Environment::Production;
            } else if self.is_github_action() {
                env = Environment::GithubActions;
            } else {
                env = Environment::Unknown;
            }
            env
        }
        fn is_local(&self) -> bool {
            let env_result = env::var("LOCAL");
            match env_result {
                Ok(val) => {
                    if val == "true" {
                        true
                    } else {
                        false
                    }
                }
                Err(_) => false,
            }
        }
        fn is_production(&self) -> bool {
            let env_result = env::var("PRODUCTION");
            match env_result {
                Ok(val) => {
                    if val == "true" {
                        true
                    } else {
                        false
                    }
                }
                Err(_) => false,
            }
        }
        fn is_github_action(&self) -> bool {
            let env_result = env::var("GITHUB_ACTIONS");
            match env_result {
                Ok(val) => {
                    if val == "true" {
                        true
                    } else {
                        false
                    }
                }
                Err(_) => false,
            }
        }
    }

    #[test]
    #[should_panic]
    fn get_env_var_should_fail() {
        let some_random_var = env::var("DOES_NOT_EXIST");
        match some_random_var {
            Ok(val) => {
                assert_eq!(val, "true");
            }
            Err(err) => {
                panic!("Error: {}", err);
            }
        }
    }

    #[test]
    fn env_type_should_not_be_unknown() {
        let env_config = EnvironmentType::new();
        assert_ne!(env_config.get_env(), Environment::Unknown);
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

    #[tokio::test]
    async fn test_async_get_from_docker() {
        let mut env_config = EnvironmentType::new();
        env_config.env = env_config.get_env();
        if env_config.env == Environment::Development {
            env_config.url = String::from("http://localhost");
            env_config.port = String::from("8000");
            env_config.route = String::from("/get");
        } else if env_config.env == Environment::Production {
            env_config.url = String::from("http://prod_url");
            env_config.port = String::from("443");
            env_config.route = String::from("/get");
        } else if env_config.env == Environment::GithubActions {
            env_config.url = String::from("http://172.17.0.2");
            env_config.port = String::from("80");
            env_config.route = String::from("/get");
        } else {
            env_config.url = String::from("https://httpbin.org");
            env_config.port = String::from("443");
            env_config.route = String::from("/get");
        }
        println!("Environment: {:?}", env_config);
        let request_url = format!(
            "{}:{}/{}",
            env_config.url, env_config.port, env_config.route
        );
        let response = reqwest::get(request_url).await;
        match response {
            Ok(res) => {
                assert_eq!(res.status().as_u16(), 200);
            }
            Err(err) => {
                panic!("Error: {}", err);
            }
        }
    }
}
