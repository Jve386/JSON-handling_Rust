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
6. [get-request](#get-request)
7. [async-await](#async-await)
8. [api-call](#api-call)
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

A simple Rust program for extracting files from a zip archive. This utility allows you to extract files safely, considering their names and permissions. 
It is built using the `zip` crate.

### Code Structure🏗️

The core functionality resides in the real_main function, which iterates over the files in a zip archive and extracts them to the specified destination. 
It considers file names and permissions for safe extraction.


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

csv-reader
=====

A basic CSV reader utility in Rust using the 'csv' crate. Handling errors with Rust's Result and the '?' operator.

### Code Structure🏗️

The 'read_from_file' function attempts to create a CSV reader from the specified file path and iterates over the records, printing them. 
Rust's 'Result' type is used to handle errors, and the '?' operator is used to propagate errors.

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


---
get-request
====
This Rust utility demonstrates making a simple GET request to the [httpbin.org](https://httpbin.org/) endpoint using the `reqwest` crate. 
It showcases error handling using the `error_chain` macro and prints the HTTP status code, headers, and response body in a readable format.

### Code Structure🏗️
```Rust
use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut response = reqwest::blocking::get("https://httpbin.org/get")?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;

    println!("Status:{}", response.status());
    println!("Headers:\n{:#?}", response.headers());
    println!("Body:\n{}", body);
    Ok(())
}

```

### Dependencies🧱
Add the following dependencies to your Cargo.toml file:
```Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
error-chain = "0.12.4"
```


### Result
```json
Status:200 OK
Headers:
{
    "date": "Sat, 20 Jan 2024 08:11:20 GMT",
    "content-type": "application/json",
    "content-length": "220",
    "connection": "keep-alive",
    "server": "gunicorn/19.9.0",
    "access-control-allow-origin": "*",
    "access-control-allow-credentials": "true",
}
Body:
{
  "args": {},
  "headers": {
    "Accept": "*/*",
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-65ab8028-523f72bb1f72c49a37cada8d"
  },
  "origin": "83.50.205.84",
  "url": "https://httpbin.org/get"
}
```

---

async-await
====
This utility  utilizes the `reqwest` crate to perform a basic HTTP GET request to the httpbin.org endpoint. 
The code employs the `error_chain` macro for effective error handling, linking to standard I/O and reqwest errors. 
It asynchronously fetches the response, prints the HTTP status code, headers in a readable format, and displays the response body. 
This example highlights Rust's asynchronous capabilities and showcases practical error handling in a concise manner.

### Code Structure🏗️
```Rust
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() ->Result<()> {
    let response = reqwest::get("http://httpbin.org/get").await?;
    println!("Status. {}", response.status());
    println!("Headers:\n{:#?}", response.headers());
    let body = response.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
```

### Dependencies🧱
Add the following dependencies to your Cargo.toml file:
```Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking", "json"] }
error-chain = "0.12.4"
tokio = { version = "1", features = ["full"] }
```


### Result
```json
Status. 200 OK
Headers:
{
    "date": "Sun, 21 Jan 2024 21:46:15 GMT",
    "content-type": "application/json",
    "content-length": "219",
    "connection": "keep-alive",
    "server": "gunicorn/19.9.0",
    "access-control-allow-origin": "*",
    "access-control-allow-credentials": "true",
}
Body:
{
  "args": {},
  "headers": {
    "Accept": "*/*",
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-65ad90a7-18b58b857ad70cd6768cab5a"
  },
  "origin": "83.50.205.84",
  "url": "http://httpbin.org/get"
}
```
---

api-call
====
This Rust utility is designed to interact with the GitHub API and retrieve the list of stargazers for a specified repository. 
It utilizes the `reqwest` crate for making HTTP requests, allowing it to communicate with the GitHub API. 
Additionally, it leverages the `serde` crate for deserializing the JSON response into Rust data structures.

### Code Structure🏗️
```Rust
use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    #[allow(dead_code)]
    login: String,
    #[allow(dead_code)]
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "mouredev",
        repo = "Pokemon-JetpackCompose",
    );
    println!("request_url:{}", &request_url);
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "Awesome-Rust-App")
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    println!("users:{:?}", users);
    Ok(())
}

```

### Dependencies🧱
Add the following dependencies to your Cargo.toml file:
```Cargo.toml
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1.35.1", features = ["full"] }

```

### Result
```json
request_url:https://api.github.com/repos/mouredev/Pokemon-JetpackCompose/stargazers
users:[User { login: "dianait", id: 18724171 }, User { login: "tortuguitahack", id: 104583107 }, User { login: "santimattius", id: 22333101 }, User { login: "EGAMAGZ", id: 46827955 }, User { login: "Corvus-DDoSKrom", id: 60022643 }, User { login: "pableArg", id: 87443200 }, User { login: "Felipeviafra", id: 116529405 }, User { login: "NitroXen", id: 92994759 }, User { login: "sergiolpzgmz", id: 95774420 }, User { login: "AzuelRei", id: 63711569 }, User { login: "gerudaeta", id: 18522826 }, User { login: "justtomartin", id: 63985401 }, User { login: "jfcorreas", id: 5447753 }, User { login: "arturoHNeoris", id: 116126750 }, User { login: "lzmdev7", id: 117466599 }, User { login: "Gozyy", id: 37049227 }, User { login: "natfme", id: 84162661 }, User { login: "johannfjs", id: 6706832 }, User { login: "RoberMiranda92", id: 7872866 }, User { login: "fcuerno001", id: 9178326 }, User { login: "hkhellou", id: 50988331 }, User { login: "GonzaCS", id: 46221344 }, User { login: "lautidias", id: 106644432 }, User { login: "juanunix", id: 20426461 }, User { login: "grostru", id: 18310362 }, User { login: "Cfcifuentesa", id: 112942635 }, User { login: "sebastianperdomo", id: 9873666 }, User { login: "pavloglez", id: 6166941 }, User { login: "emedp", id: 14973053 }, User { login: "santimb96", id: 74008042 }]
```

---

If you wanna format your code to enhance your reading use the following command:
```Rust
rustfmt main.rs
```
It will help you with possible issues before building the project.

*Note:  In the context of HTTP status codes, a status code of 200 indicates that the request has been successful. 

---

### 📖 Documentation
<a href="https://doc.rust-lang.org/book/">The Rust Programming Language</a>.

<a href="https://rust-lang-nursery.github.io/rust-cookbook/">Cookin' with Rust - Examples</a>.

<a href="https://crates.io/crates/serde">Documentation for serde</a>.

<a href="https://crates.io/crates/flate2">Documentation for flate2.</a>.

<a href="https://crates.io/crates/zip">Documentation for zip</a>.

<a href="https://crates.io/crates/csv">Documentation for csv</a>.

<a href="https://crates.io/crates/tokio">Documentation for tokio</a>.

<a href="https://crates.io/crates/reqwest">Documentation for reqwest</a>.

<a href="https://crates.io/crates/error-chain">Documentation for error-chain</a>.


--- 

I have included comprehensive comments for each function, providing detailed explanations and documentation to facilitate understanding and usage.
🌐 Feel free to contribute to this project, open issues, or use it as a learning resource for Rust development. 🤝
