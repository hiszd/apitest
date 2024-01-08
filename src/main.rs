mod db;

#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    description: &'r str,
    complete: bool,
}

#[post("/todo", data = "<task>")]
fn new(task: Json<Task<'_>>) -> String {
    format!(
        "{} {}\n",
        "Hello, world!\nThis is a description: ", task.description
    )
}

#[get("/test")]
fn ret() -> Json<Task<'static>> {
    Json(Task {
        description: "Hello, world!",
        complete: true,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![new, ret])
}
