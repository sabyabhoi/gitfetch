use std::error::Error;

use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

const ENDPOINT: &'static str = "https://api.github.com/";

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    avatar_url: String,
    public_repos: u32,
    followers: u32,
    following: u32,
}

impl User {
    async fn from(username: &str) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let res = client
            .get(ENDPOINT.to_owned() + "users/" + username)
            .header(USER_AGENT, "sabyabhoi")
            .send()
            .await?;

        Ok(serde_json::from_str::<User>(res.text().await?.as_str())?)
    }
}

#[tokio::main]
async fn main() {
    let me = User::from("sabyabhoi").await.unwrap();
}
