use serde::Deserialize;

#[derive(Clone, PartialEq)]
pub struct Repo {
    pub name: String,
    pub html_url: String,
    pub language: String
}