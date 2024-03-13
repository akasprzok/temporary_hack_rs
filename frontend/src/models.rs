use serde::Deserialize;

pub struct Repo {
    pub name: String,
    pub html_url: String,
    pub language: String,
}
