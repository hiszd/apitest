use diesel::prelude::*;

mod db;
pub mod model;
pub mod schema;

use crate::model::*;
use crate::schema::*;
use apitest::establish_connection;

#[macro_use]
extern crate rocket;

// use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    description: &'r str,
    complete: bool,
}
#[get("/user/new/<name>")]
fn newuser(name: &str) -> String {
    let conn = &mut establish_connection();
    let user = diesel::insert_into(users::table)
        .values(users::name.eq(name))
        .returning(User::as_returning())
        .get_result(conn)
        .unwrap();
    format!("User: {:?}\n", user)
}

#[get("/user/get/<name>")]
fn getuser(name: &str) -> String {
    let conn = &mut establish_connection();
    let user = users::table
        .filter(users::name.eq(name))
        .select(User::as_select())
        .get_result(conn);
    if user.is_err() {
        return String::from("User not found");
    } else {
        format!("User: {:?}\n", user.unwrap())
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![newuser, getuser])
}
