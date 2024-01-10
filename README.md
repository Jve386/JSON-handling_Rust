# Rust projects

If you encounter issues with 'Terminal' prompting, grant permissions using the following command:
```bash
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
```

## Table of Contents:
1. [Read Json Data](#read-json-data)
2. [Compressor](#compressor)
3. [Decompressor](#decompressor)

---

## JSON Reader (read-json) 

This repository implements a basic JSON reader in Rust, leveraging the serde and serde_json dependencies. 

### Code Structure🏗️
The project defines two data structures, Paragraph and Article, annotated with Serialize and Deserialize traits from serde. 
The main function demonstrates parsing a predefined JSON string into an Article object and accessing its properties.

<<<<<<< HEAD
<p align="center"><a href="/img/class_diagram.png" target="_blank">
    <img src="/img/rust_1.jpg" />
  </a></p>
  
Dependencies:


<p align="center"><a href="/img/class_diagram.png" target="_blank">
    <img src="/img/rust_2.jpg" />
</a></p>
=======
```Rust
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
let parsed: Article = read_json_typed(json);

// Print the name of the first paragraph to the console
println!("\n\n The name of the first paragraph is: {}", parsed.paragraph[0].name);

```
  
 ### Dependencies🧱
Add the following dependencies to your Cargo.toml file:
```Cargo.toml
[dependencies]
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
```
>>>>>>> zip-extract

---

## Compressor (zip-create)

- To be filled. Work in progress...

---

## Decompressor (zip-extract)


A simple Rust program for extracting files from a zip archive. This utility allows you to extract files safely, considering their names and permissions. It is built using the `zip` crate.

### Code Structure🏗️

The core functionality resides in the real_main function, which iterates over the files in a zip archive and extracts them to the specified destination. It considers file names and permissions for safe extraction.


```Rust
// ... (imports)

fn real_main() -> i32 {
    // ... (argument parsing)

    for i in 0..archive.len() {
        // ... (extracting files)
    }

    0 // Return 0 to indicate successful program execution
}

fn main() {
    std::process::exit(real_main())
}

```

### Dependencies🧱
Add the following dependencies to your Cargo.toml file:
```
zip = "0.6.2"
```

### Usage

To use the `zip-extract` utility, provide the filename of the zip archive as a command-line argument. For example:

```bash
cargo run your_archive.zip
```

---
I have included comprehensive comments for each function, providing detailed explanations and documentation to facilitate understanding and usage.
🌐 Feel free to contribute to this project, open issues, or use it as a learning resource for Rust development. 🤝
