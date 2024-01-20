// Import necessary libraries and macros
use error_chain::error_chain;
use std::io::Read;

// Define custom error types using the error_chain macro
error_chain! {
    foreign_links {
        Io(std::io::Error);            // Link to std::io::Error for I/O operations
        HttpRequest(reqwest::Error);   // Link to reqwest::Error for HTTP requests
    }
}

// Entry point of the program
fn main() -> Result<()> {
    // Make a GET request to the specified URL
    let mut response = reqwest::blocking::get("https://httpbin.org/get")?;

    // Create a string to store the response body
    let mut body = String::new();

    // Read the response body into the 'body' string
    response.read_to_string(&mut body)?;

    // Print the HTTP status code of the response
    println!("Status: {}", response.status());

    // Print the headers of the response in a formatted way
    println!("Headers:\n{:#?}", response.headers());

    // Print the body of the response
    println!("Body:\n{}", body);

    // Return a Result indicating success
    Ok(())
}
