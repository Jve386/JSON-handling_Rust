# Rust projects

If you encounter issues with 'Terminal' prompting, grant permissions using the following command:
```bash
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
```

## Table of Contents:
1. [read-json ](#read-json)
2. [zip-create](#zip-create)
3. [zip-extract](#zip-extract)


---

read-json 
=====

This project implements a basic JSON reader in Rust, leveraging the serde and serde_json dependencies. 

### Code Structure🏗️
The project defines two data structures, Paragraph and Article, annotated with Serialize and Deserialize traits from serde. 
The main function demonstrates parsing a predefined JSON string into an Article object and accessing its properties.

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

---

 
zip-create
=====
This project utilizes the flate2 crate to provide a simple solution for compressing files.

### Code Structure🏗️

This program takes a source file, compresses it, and saves the compressed result to a target file.

```Rust
extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::{env::args, fs::File, io::{self, BufReader}, time::Instant};

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());

    io::copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    // Display source and target lengths, and time elapsed
    println!("Source len: {:?}", input.get_ref().metadata().unwrap().len());
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Time elapsed: {:?}", Instant::now().elapsed());
}

```

### Dependencies🧱
Add the following dependencies to your Cargo.toml file:

```bash
[dependencies]
flate2 = "1.0.28"
```

### Usage
To use the zip-create utility, provide the source and target filenames as command-line argument:
```bash
cargo run source_file.txt target_file.zip
```

---

zip-extract
=====

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
