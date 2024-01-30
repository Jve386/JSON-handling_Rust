// Import necessary libraries
use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

// Define custom error types using the error_chain macro
error_chain! {
    foreign_links {
        Reqerror(reqwest::Error);
        IoError(std::io::Error);
    }
}

// Define the main asynchronous function
#[tokio::main]
async fn main() -> Result<()> {
    // Make a GET request to the Rust Lang website and retrieve the text content
    let res = reqwest::get("https://www.rust-lang.org/en-US/").await?.text().await?;

    // Parse the HTML content into a Document and find all 'a' tags
    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    // Return a successful result
    Ok(())
}
