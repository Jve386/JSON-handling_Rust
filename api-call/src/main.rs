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
