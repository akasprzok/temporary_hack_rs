
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Greeting {
    pub text: &'static str,
}

impl Greeting {
    pub fn new() -> Greeting {
        Greeting {
            text: Self::default_greeting()
        }
    }
    fn default_greeting() -> &'static str {
        "Hello, World!"
    }
}