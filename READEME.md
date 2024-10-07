# Rust GitHub API Stargazers Fetcher

A simple Rust command-line program to fetch the list of stargazers (users who starred a repository) from the GitHub API.

## Features
- Fetches stargazers from any GitHub repository.
- Asynchronous HTTP requests using the `reqwest` library.
- Automatically parses the API response using `serde`.

## Usage

You can easily modify the repository and owner that the script fetches data from by changing the `owner` and `repo` variables inside the `main()` function.

Example:
```rust
let owner = "rust-lang";
let repo = "rust";
