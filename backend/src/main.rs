#[macro_use]
extern crate rocket;

use std::error::Error;

use github::Repo;
use rocket::response::status;
use rocket::{serde::json::Json, Request, State};

mod github;
mod greetings;

use greetings::Greeting;

use crate::github::GitHubClient;

#[get("/")]
fn index() -> Json<Greeting> {
    let greeting = Greeting::new();
    Json(greeting)
}

#[get("/repos")]
async fn repos(
    github_client: &State<GitHubClient>,
) -> Result<Json<Vec<Repo>>, status::NotFound<String>> {
    match github_client.repos().await {
        Ok(repos) => Ok(Json(repos)),
        Err(e) => Err(status::NotFound(e.to_string())),
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let github_client = github::GitHubClient::new()?;

    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index, repos])
        .manage(github_client)
        .launch()
        .await?;

    Ok(())
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Woops, couldn't find '{}", req.uri())
}
