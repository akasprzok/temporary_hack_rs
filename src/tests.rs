use super::rocket;

use rocket::local::blocking::Client;

fn test_root(kind: &str) {
    let client = Client::tracked(rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}