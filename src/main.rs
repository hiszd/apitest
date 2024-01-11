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

#[get("/ticket/get/by/author/<author_id>", rank = 2)]
fn get_ticket_by_author(author_id: i32) -> String {
    let conn = &mut establish_connection();
    let authorraw = users::table
        .filter(users::id.eq(author_id))
        .select(User::as_select())
        .get_result(conn);
    if authorraw.is_err() {
        return String::from("author not found");
    }

    let author = authorraw.unwrap();

    let tickets = TicketAuthor::belonging_to(&author)
        .inner_join(tickets::table)
        .select(Ticket::as_select())
        .load(conn);
    if tickets.is_err() {
        return String::from("error thing");
    } else {
        format!("Tickets: {:?}\n", tickets.unwrap())
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![newuser, getuser, get_ticket_by_author])
}
