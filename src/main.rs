#[macro_use] extern crate rocket;

use rocket::{serde::{json::Json}, Request};

mod greetings;

use greetings::Greeting;
use rocket_dyn_templates::{context, Template};

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

#[launch]
fn rocket () -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .mount("/", routes![index, index_json, hello])
        .attach(Template::fairing())
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Woops, couldn't find '{}", req.uri())
}