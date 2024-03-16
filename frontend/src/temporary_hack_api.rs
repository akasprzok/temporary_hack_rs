use common::model::repo::Repo;
use reqwasm::{http::Request, Error};

const BASE_URL: &str = "http://localhost:8000";

pub async fn fetch_repos() -> Result<Vec<Repo>, Error> {
    Request::get(&format!("{BASE_URL}/repos"))
        .send()
        .await?
        .json()
        .await
}
