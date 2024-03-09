#[macro_use] extern crate rocket;

use std::error::Error;

use rocket::{serde::json::Json, Request, State};

mod greetings;
mod github;

use greetings::Greeting;
use rocket_dyn_templates::{context, Template};

use crate::github::GitHubClient;

#[get("/", format = "json")]
fn index_json() -> Json<Greeting> {
    let greeting = Greeting::new();
    Json(greeting)
}

#[get("/", rank = 1)]
fn index() -> Template {
    Template::render("index", context! {
        title: "hello",
        name: Some("Chris"),
        items: vec!["one"]
    })
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/repos")]
async fn repos(github_client: &State<GitHubClient>) -> String {
    let result = github_client.repos().await;
    match result {
        Ok(response) => format!("response: {:?}", response),
        Err(e) => format!("Error! {:?}", e)
    }
    
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let github_client = github::GitHubClient::new()?;

    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index, index_json, hello, repos])
        .attach(Template::fairing())
        .manage(github_client)
        .launch()
        .await?;
    
    Ok(())
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Woops, couldn't find '{}", req.uri())
}