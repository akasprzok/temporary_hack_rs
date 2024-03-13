use reqwest::{header, Client};

use common::model::repo::Repo;

pub struct GitHubClient {
    client: Client,
    username: String,
    password: String,
}

impl GitHubClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Accept",
            header::HeaderValue::from_static("application/vnd.github+json"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .user_agent("TemporaryHack")
            .build()?;

        let username = std::env::var("GITHUB_USER").expect("GITHUB_USER should exist");
        let password = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN should exist");

        Ok(GitHubClient {
            client,
            username,
            password,
        })
    }

    pub async fn repos(&self) -> Result<Vec<Repo>, reqwest::Error> {
        let response = self
            .client
            .get("https://api.github.com/user/repos")
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;

        println!("{:?}", &response.headers());

        response.json::<Vec<Repo>>().await
    }
}
