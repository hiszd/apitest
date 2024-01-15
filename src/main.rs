use diesel::prelude::*;

mod db;

use apitest::establish_connection;
use apitest::model::*;
use apitest::schema::*;
use apitest::types::{statustype::StatusType, tickettype::TicketType};

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;

#[catch(default)]
fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})\n{:?}", status, req.uri(), req.headers());
    status::Custom(status, msg)
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct TicketJson {
    count: i32,
    subject: String,
    description: String,
    ticktype: String,
    status: String,
}

#[get("/ticket/list")]
fn listtickets() -> Json<Vec<Ticket>> {
    let conn = &mut establish_connection();
    let tik = tickets::table.select(Ticket::as_select()).load(conn);

    match tik.is_err() {
        true => Json(Vec::new()),
        false => Json(tik.unwrap()),
    }
}

#[get("/ticket/del/<id>")]
fn delticket(id: i32) -> String {
    let conn = &mut establish_connection();
    let del = diesel::delete(tickets::table.filter(tickets::id.eq(id))).execute(conn);

    match del.is_err() {
        true => String::from("Error"),
        false => String::from("Success"),
    }
}

#[post("/ticket/new", data = "<ticket>")]
fn newticket(ticket: Json<TicketJson>) -> Option<Json<Ticket>> {
    let conn = &mut establish_connection();
    let tik = diesel::insert_into(tickets::table)
        .values((
            tickets::count.eq(ticket.count),
            tickets::subject.eq(&ticket.subject),
            tickets::description.eq(&ticket.description),
            tickets::status.eq(StatusType::from(ticket.status.clone())),
            tickets::ticktype.eq(TicketType::from(ticket.ticktype.clone())),
        ))
        .returning(Ticket::as_returning())
        .get_result(conn);

    match tik.is_err() {
        true => None,
        false => Some(Json(tik.unwrap())),
    }
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

    let tickets = tickets_authors::table
        .filter(tickets_authors::author_id.eq(author_id))
        .inner_join(tickets::table)
        .select(TicketAuthor::as_select())
        .load(conn);

    // let tickets = TicketAuthor::belonging_to(&author)
    //     .inner_join(tickets::table)
    //     .select(Ticket::as_select())
    //     .load(conn);
    if tickets.is_err() {
        return String::from("Tickets not found");
    } else {
        format!("Tickets: {:?}\n", tickets.unwrap())
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                newuser,
                getuser,
                get_ticket_by_author,
                newticket,
                listtickets,
                delticket
            ],
        )
        .register("/", catchers![default_catcher])
}
