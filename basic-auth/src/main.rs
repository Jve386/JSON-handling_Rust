// Import necessary modules from reqwest library
use reqwest::blocking::Client;
use reqwest::Error;

// Main function returning a Result indicating success or an error
fn main() -> Result<(), Error> {
    // Create a new reqwest Client for making HTTP requests
    let client = Client::new();

    // Define a test username
    let user = "testuser".to_string();
    // Define an optional test password (None for no password)
    let passwd: Option<String> = None;

    // Make a GET request to "https://httpbin.org/get" with basic authentication
    let res = client
        .get("https://httpbin.org/get")
        .basic_auth(user, passwd)
        .send();

    // Print the response for debugging purposes
    println!("{:?}", res);

    // Return Ok to indicate successful execution of the program
    Ok(())
}
