#[cfg(test)]
mod write_json_tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize)]
    struct A<'a> {
        a: &'a str,
        b: &'a str,
    }
    #[derive(Serialize, Deserialize)]
    struct B<'z> {
        x: &'z str,
        y: [A<'z>; 2],
    }

    #[test]
    fn it_writes_json_from_hashmap() {
        let mut map = HashMap::new();
        map.insert("name", "John");
        map.insert("age", "30");
        map.insert("city", "New York");

        let json = json!(map);
        assert_eq!(
            json.to_string(),
            r#"{"age":"30","city":"New York","name":"John"}"#
        );
    }

    #[test]
    fn it_writes_json() {
        let json_to_serialize = B {
            x: "x",
            y: [A { a: "x", b: "y" }, A { a: "p", b: "q" }],
        };
        let serialized_json = serde_json::to_string(&json_to_serialize);
        match serialized_json {
            Ok(json) => assert_eq!(
                json,
                r#"{"x":"x","y":[{"a":"x","b":"y"},{"a":"p","b":"q"}]}"#
            ),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
