use crate::models::*;

const BASE_URL: &str = "http://localhost:8080";

pub async fn fetch_repos() -> Result<Vec<Rep>, Error> {
    Request::get(&format!("{BASE_URL}/repos"))
        .send()
        .await?
        .json()
        .await
}