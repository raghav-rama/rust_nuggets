use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person<'a> {
    name: &'a str,
    country: &'a str,
    projects: [Projects<'a>; 2],
}

#[derive(Serialize, Deserialize)]
struct Projects<'a> {
    name: &'a str,
    description: &'a str,
}

fn main() {
    let person = Person {
        name: "John Doe",
        country: "USA",
        projects: [
            Projects {
                name: "Project 1",
                description: "This is project 1",
            },
            Projects {
                name: "Project 2",
                description: "This is project 2",
            },
        ],
    };
    let json_result = serde_json::to_string(&person);
    match json_result {
        Ok(json) => println!("{}", json),
        Err(e) => println!("Error: {}", e),
    }
}
