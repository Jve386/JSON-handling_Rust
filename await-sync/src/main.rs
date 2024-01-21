// Import the error_chain crate to simplify error handling.
use error_chain::error_chain;

// Define custom error types using the error_chain macro.
error_chain! {
    // Define foreign links to link to external error types.
    foreign_links {
        Io(std::io::Error);           // Link to std::io::Error for I/O operations.
        HttpRequest(reqwest::Error);  // Link to reqwest::Error for HTTP request errors.
    }
}

// Define the main function as an asynchronous function using #[tokio::main].
#[tokio::main]
async fn main() -> Result<()> {
    // Make an asynchronous HTTP GET request to httpbin.org/get.
    let response = reqwest::get("http://httpbin.org/get").await?;

    // Print the status code of the HTTP response.
    println!("Status: {}", response.status());

    // Print the headers of the HTTP response in a pretty format.
    println!("Headers:\n{:#?}", response.headers());

    // Read the body of the HTTP response as text and print it.
    let body = response.text().await?;
    println!("Body:\n{}", body);

    // Return a Result indicating success.
    Ok(())
}
