# Rust projects

If you encounter issues with 'Terminal' prompting, grant permissions using the following command:
```bash
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
```

## Table of Contents:
1. [read-json ](#read-json)
2. [zip-create](#zip-create)
3. [zip-extract](#zip-extract)
4. [csv-reader](#csv-reader)
5. [write-json](#write-json)

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
<a href="https://crates.io/crates/serde">Documentation for serde.</a> 

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
<a href="https://crates.io/crates/flate2">Documentation for flate2.</a> 

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
<a href="https://crates.io/crates/zip">Documentation for zip.</a>  

### Usage

To use the `zip-extract` utility, provide the filename of the zip archive as a command-line argument. For example:

```bash
cargo run your_archive.zip
```

---

csv-reader
=====

A basic CSV reader utility in Rust using the 'csv' crate. Handling errors with Rust's Result and the '?' operator.

### Code Structure🏗️

The 'read_from_file' function attempts to create a CSV reader from the specified file path and iterates over the records, printing them. Rust's 'Result' type is used to handle errors, and the '?' operator is used to propagate errors.

```Rust
use std::error::Error;
use csv;

// Function to read and print records from a CSV file
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    // Attempt to create a CSV reader from the file
    let mut reader = csv::Reader::from_path(path)?;

    // Iterate over records in the CSV reader
    for result in reader.records() {
        // Attempt to get a record from the result
        let record = result?;

        // Print the record
        println!("{:?}", record);
    }

    Ok(()) // Return Ok if there is no error
}

// Main function
fn main() {
    // Attempt to read from the "sample.csv" file
    if let Err(e) = read_from_file("./sample.csv") {
        // Print the error if there is one
        eprintln!("{}", e);
    }
}

```

You have tons of CSV samples <a href="https://wsform.com/knowledgebase/sample-csv-files/">here</a>. 

### Dependencies🧱
Add the following dependencies to your Cargo.toml file:
```
csv = "1.3"
```
<a href="https://crates.io/crates/csv">Documentation for csv.</a>  

### Usage

To use the 'csv-reader' just make sure to have the csv file with the exact name that you have in the function - in this case, it is sample.csv:
```bash
cargo run
```

---
write-json
====

This utility demonstrates the use of the `serde` library in Rust for JSON serialization and deserialization.

### Code Structure🏗️

Serialize and deserialize JSON data for two structs: `Person` and `Company`. 
Convert a string to `u8` for the `age` field in the `Person` struct.

```Rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Company {
    name: String,
    address: String,
}

fn main() {
    let person: Person = Person {
        name: String::from("Javier de la Vega"),
        age: "30".parse().expect("Failed to parse age"),
        phones: vec![String::from("555-1234"), String::from("555-4321")],
    };

    let company: Company = Company {
        name: String::from("Jve386"),
        address: String::from("Street 123"),
    };

    let json = serde_json::to_string(&person).unwrap();
    println!("Person: {}", json);
    let json = serde_json::to_string(&company).unwrap();
    println!("Company: {}", json);
}

```

 ### Dependencies🧱
Add the following dependencies to your Cargo.toml file:
```Cargo.toml
[dependencies]
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
```
<a href="https://crates.io/crates/serde">Documentation for serde.</a> 

---


If you wanna format your code to enhance your reading use the following command:
```Rust
rustfmt main.rs
```
It will help you with possible issues before building the project.

--- 

I have included comprehensive comments for each function, providing detailed explanations and documentation to facilitate understanding and usage.
🌐 Feel free to contribute to this project, open issues, or use it as a learning resource for Rust development. 🤝
