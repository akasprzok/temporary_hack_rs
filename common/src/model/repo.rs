use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct License {
    spdx_id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Repo {
    name: String,
    html_url: String,
    fork: bool,
    stargazers_count: u16,
    watchers_count: u16,
    language: Option<String>,
    license: Option<License>,
    topics: Vec<String>,
}
