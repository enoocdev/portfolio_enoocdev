use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GithubUser {
    pub login: String,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GithubRepo {
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: u32,
    pub fork: bool,
}

pub async fn get_user_data(client: &Client, username: &str) -> Option<GithubUser> {
    let url = format!("https://api.github.com/users/{}", username);
    match client.get(&url).header("User-Agent", "enoocdev-portfolio").send().await {
        Ok(resp) => resp.json().await.ok(),
        Err(_) => None,
    }
}

pub async fn get_user_repos(client: &Client, username: &str) -> Vec<GithubRepo> {
    let url = format!("https://api.github.com/users/{}/repos?per_page=100", username);
    match client.get(&url).header("User-Agent", "enoocdev-portfolio").send().await {
        Ok(resp) => resp.json().await.unwrap_or_else(|_| vec![]),
        Err(_) => vec![],
    }
}