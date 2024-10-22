use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

fn main() {
    // Read the JSON file
    let json_str = std::fs::read_to_string("./token.definition.json")
        .expect("Failed to read the file.");

    // Deserialize the JSON into a Person struct
    let person: Person = serde_json::from_str(&json_str)
        .expect("Failed to deserialize JSON.");

    // Print the deserialized object 
    println!("{:?}", person);
}