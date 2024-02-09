use serde::{Deserialize, Serialize};

fn main() {
    let dummy_json = r#"
    {
        "name": "John Doe",
        "country": "USA",
        "number": "123",
        "projects": [
            {
                "name": "Project 1",
                "description": "Description 1"
            },
            {
                "name": "Project 2",
                "description": "Description 2"
            }
        ]
    }
    "#;
    let parsed = serde_json::from_str::<Person>(dummy_json);
    // can also use unwrap instead of match
    match parsed {
        Ok(ref person) => {
            println!("Name: {}", person.name);
            println!("Country: {}", person.country);
            println!("Number: {}", person.number);
            for project in person.projects.iter() {
                println!("Project Name: {}", project.name);
                println!("Project Description: {}", project.description);
            }
        }
        Err(ref e) => {
            println!("Error: {}", e);
        }
    }
    print!("\n");
    if let Ok(json) = serde_json::to_string(match parsed {
        Ok(ref person) => person,
        Err(ref e) => {
            println!("Error: {}", e);
            return;
        }
    }) {
        println!("Serialized JSON: {}", json);
    }
}

#[derive(Serialize, Deserialize)]
struct Person<'a> {
    // lifetimes are necessary because we are using references (borrowed string)
    name: &'a str,
    country: &'a str,
    number: &'a str,
    projects: [Project<'a>; 2], // specify size because serde can't serialize arrays with dynamic size
}

#[derive(Serialize, Deserialize)]
struct Project<'a> {
    name: &'a str,
    description: &'a str,
}
