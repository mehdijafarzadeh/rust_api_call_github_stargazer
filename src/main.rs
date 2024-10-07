use reqwest::Error;
use reqwest::header::USER_AGENT;
use serde::{Serialize, Deserialize};

/// A struct representing a GitHub user (stargazer)
#[derive(Debug, Serialize, Deserialize)]
struct User {
    login: String,  // GitHub username
    id: u32,        // GitHub user ID
}

/// Main asynchronous function to fetch stargazers of a given GitHub repository
///
/// # Arguments
/// - `owner`: The owner of the GitHub repository
/// - `repo`: The repository name
///
/// # Returns
/// A `Result` containing either an empty tuple on success or a `reqwest::Error` on failure.
#[tokio::main]
async fn main() -> Result<(), Error> {
    // GitHub repository owner and repo name
    let owner = "rust-lang";
    let repo = "rust";

    // Format the API request URL
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers");

    println!("Requesting stargazers from: {}", request_url);

    // Create an HTTP client
    let client = reqwest::Client::new();

    // Make a GET request to the GitHub API with a custom User-Agent header
    let response = client.get(&request_url)
        .header(USER_AGENT, "rust-web-api-client-demo") // GitHub requires a custom User-Agent
        .send()
        .await?; // Send the request asynchronously and handle any potential errors

    // Parse the JSON response into a vector of User structs
    let users: Vec<User> = response.json().await?;

    // Print the list of stargazers (GitHub users who starred the repo)
    println!("Stargazers: {:?}", users);

    Ok(())
}
