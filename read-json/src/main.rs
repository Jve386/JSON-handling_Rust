// Import necessary traits from the serde library for JSON serialization and deserialization
use serde::{Deserialize, Serialize};

// Define a struct 'Paragraph' with a single field 'name' of type String
#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

// Define a struct 'Article' with fields 'article' and 'author' of type String,
// and 'paragraph' as a vector of 'Paragraph' structs
#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

// Define the main function
fn main() {
    // A JSON string representing an 'Article' object
    let json = r#"
    {
        "article": "how to work with JSON in Rust",
        "author": "Jve386",
        "paragraph": [
            {
                "name": "starting sentence"
            },
            {
                "name": "body of the paragraph"
            },
            {
                "name": "ending paragraph"
            }
        ]
    }"#;

    // Deserialize the JSON string into an 'Article' object
    // Note: The 'read_json_typed' function needs to be implemented or replaced with the appropriate deserialization function
    let parsed: Article = read_json_typed(json);

    // Print the name of the first paragraph to the console
    println!("\n\n The name of the first paragraph is: {}", parsed.paragraph[0].name);
}

fn read_json_typed(raw_json : &str) -> Article{
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    parsed
}