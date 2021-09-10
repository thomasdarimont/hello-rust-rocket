#[macro_use] extern crate rocket;

use chrono::{Utc};

#[get("/")]
fn index() -> String {
    format!("Hello, {}! Time: {}", "world", Utc::now().format("%a %b %e %T %Y")).to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}