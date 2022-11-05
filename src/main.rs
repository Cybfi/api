#[macro_use] extern crate rocket;
use rocket::serde::{Deserialize, json::Json};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]

/* Structures for Responses */
struct V2response<'r> {
    status: &'r i8,
    complete: bool
}

/* Endpoints */
#[get("/")]
fn index() -> &'static str {
    "https://api.cybfi.net/v2"
}

#[get("/v2")]
fn v2() -> &'static str {
    Json(v2Response(200, true))
}

#[launch]
fn main() -> _ {
    rocket::build().mount("/", routes![index])
}