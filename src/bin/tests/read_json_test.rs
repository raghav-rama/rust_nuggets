#[cfg(test)]
mod read_json_test {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Serialize, Deserialize)]
    struct B<'b> {
        c: &'b str,
        d: [A<'b>; 2],
    }
    #[derive(Serialize, Deserialize)]
    struct A<'a> {
        a: &'a str,
        b: &'a str,
    }
    #[test]
    fn it_reads_the_json() {
        if let Some(dummy_json) = json!({
            "a": "some a",
            "b": "some b",
        })
        .as_str()
        {
            let parsed = serde_json::from_str::<A>(dummy_json);
            match parsed {
                Ok(ref a) => {
                    assert_eq!(a.a, "some a");
                    assert_eq!(a.b, "some b");
                }
                Err(ref e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}
