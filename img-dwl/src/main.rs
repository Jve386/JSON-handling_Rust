// Import necessary libraries and macros
use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;

// Define custom error types using the error_chain macro
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

// Define the main asynchronous function
#[tokio::main]
async fn main() -> Result<()> {
    // Specify the desired download directory
    let download_dir = "C:\\";

    // Create a temporary directory in the specified download directory
    let tmp_dir = Builder::new().prefix("img-dwl").tempdir_in(download_dir)?;

    // Define the URL of the file to be downloaded
    let target = "https://www.rust-lang.org/static/images/rust-logo-blk.svg";

    // Send an HTTP GET request to the specified URL and await the response
    let res = reqwest::get(target).await?;

    // Check if the HTTP request was successful (status code in the 2xx range)
    if !res.status().is_success() {
        // Print an error message and return an Err variant
        eprintln!("Failed to download: {:?}", res.status());
        return Err("Failed to download".into());
    }

    // Extract the file name from the URL
    let fname = res
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");

    // Print the name of the file to be downloaded
    println!("file to download: '{}'", fname);

    // Create the full file path by joining the temporary directory and the file name
    let file_path = tmp_dir.into_path().join(fname);
    // Print the location where the file will be saved
    println!("Will be located under {:?}", file_path);

    // Create a new file at the specified path
    let mut dest = match File::create(&file_path) {
        Ok(file) => file,
        Err(e) => {
            // Print an error message and return an Err variant
            eprintln!("Error creating file: {:?}", e);
            return Err(e.into());
        }
    };

    // Read the content of the HTTP response body as bytes
    let content = res.bytes().await?;
    // Copy the content to the newly created file
    copy(&mut content.as_ref(), &mut dest)?;

    // Print a success message
    println!("File downloaded successfully!");

    // Return an Ok variant to indicate success
    Ok(())
}