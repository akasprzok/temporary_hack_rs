use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct License {
    pub spdx_id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Repo {
    pub name: String,
    pub html_url: String,
    pub fork: bool,
    pub stargazers_count: u16,
    pub watchers_count: u16,
    pub language: Option<String>,
    pub license: Option<License>,
    pub topics: Vec<String>,
}
