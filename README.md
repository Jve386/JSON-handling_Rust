# Read Json Data - Rust 

This repository implements a basic JSON reader in Rust, leveraging the serde and serde_json dependencies. 

The project defines two data structures, Paragraph and Article, annotated with Serialize and Deserialize traits from serde. 
The main function demonstrates parsing a predefined JSON string into an Article object and accessing its properties.

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"
    {
        "article": "Read Json with Rust",
        "author": "Jve386",
        "paragraph": [
            {
                "name": "starting sentence"
            },
            {
                "name": "body"
            },
            {
                "name": "ending sentence"
            }
        ]
    }"#;

    let parsed: Article = read_json_typed(json);
    println!("\n\n The name of the first paragraph is: {}", parsed.paragraph[0].name);
}
```
Dependencies:

```rust
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
```

Feel free to contribute, open issues, or use this project as a learning resource for Android development and API integration. The project leverages modern Android development practices and provides insights into using View Binding, Coroutines, Retrofit, and Picasso.
