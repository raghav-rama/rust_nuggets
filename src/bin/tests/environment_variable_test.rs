#[cfg(test)]
mod environment_variable_test {
    use std::env;

    #[test]
    #[should_panic]
    fn it_gets_environment_variable() {
        let key = "X";
        let value = env::var(key);
        match value {
            Ok(val) => {
                assert_eq!(val, "true");
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }
}
