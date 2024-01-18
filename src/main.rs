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
use rocket_dyn_templates::Template;

#[catch(default)]
fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})\n{:?}", status, req.uri(), req.headers());
    status::Custom(status, msg)
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct UserJson {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct NewTicketJson {
    author_id: i32,
    count: i32,
    subject: String,
    description: String,
    ticktype: String,
    status: String,
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

#[post("/user/new", data = "<user>")]
fn new_user(user: Json<UserJson>) -> Json<User> {
    let usr = diesel::insert_into(users::table)
        .values(users::name.eq(&user.name))
        .returning(User::as_returning())
        .get_result(&mut establish_connection());
    Json(usr.unwrap())
}

#[get("/user/<id>")]
fn get_user_by_id(id: i32) -> Json<User> {
    Json(
        users::table
            .filter(users::id.eq(id))
            .select(User::as_select())
            .get_result(&mut establish_connection())
            .unwrap(),
    )
}

#[get("/user/<name>", rank = 2)]
fn get_user_by_name(name: &str) -> Json<User> {
    Json(
        users::table
            .filter(users::name.eq(name))
            .select(User::as_select())
            .get_result(&mut establish_connection())
            .unwrap(),
    )
}

#[get("/user/list/all")]
fn list_users() -> Json<Vec<User>> {
    Json(
        users::table
            .select(User::as_select())
            .load(&mut establish_connection())
            .unwrap(),
    )
}

#[get("/user/remove/<id>")]
fn remove_user_by_id(id: i32) -> Option<Json<User>> {
    let conn = &mut establish_connection();
    let usr = users::table
        .filter(users::id.eq(id))
        .select(User::as_select())
        .get_result(conn);
    let del = diesel::delete(users::table)
        .filter(users::id.eq(id))
        .execute(conn);
    if del.is_err() || usr.is_err() {
        None
    } else {
        Some(Json(usr.unwrap()))
    }
}

#[post("/ticket/new", data = "<ticket>")]
fn new_ticket(ticket: Json<NewTicketJson>) -> Json<Ticket> {
    // create table entry to tickets for the new ticket
    let tik = diesel::insert_into(tickets::table)
        .values((
            tickets::count.eq(ticket.count),
            tickets::subject.eq(&ticket.subject),
            tickets::description.eq(&ticket.description),
            tickets::status.eq(StatusType::from(ticket.status.clone())),
            tickets::ticktype.eq(TicketType::from(ticket.ticktype.clone())),
        ))
        .returning(Ticket::as_returning())
        .get_result(&mut establish_connection())
        .unwrap();

    // create table entry to tickets_authors for the relation between the ticket and the author
    diesel::insert_into(tickets_authors::table)
        .values((
            tickets_authors::author_id.eq(ticket.author_id),
            tickets_authors::ticket_id.eq(tik.id),
        ))
        .execute(&mut establish_connection())
        .unwrap();
    Json(tik)
}

#[get("/ticket/<id>")]
fn get_ticket_by_id(id: i32) -> Json<Ticket> {
    Json(
        tickets::table
            .filter(tickets::id.eq(id))
            .select(Ticket::as_select())
            .get_result(&mut establish_connection())
            .unwrap(),
    )
}

#[get("/ticket/list/<author_id>")]
fn get_tickets_by_author_id(author_id: i32) -> Json<Vec<Ticket>> {
    let author = users::table
        .filter(users::id.eq(author_id))
        .select(User::as_select())
        .get_result(&mut establish_connection())
        .unwrap();
    Json(
        tickets_authors::table
            .filter(tickets_authors::author_id.eq(author.id))
            .inner_join(tickets::table)
            .select(Ticket::as_select())
            .load(&mut establish_connection())
            .unwrap(),
    )
}

#[get("/ticket/list/all")]
fn list_tickets() -> Json<Vec<Ticket>> {
    Json(
        tickets::table
            .select(Ticket::as_select())
            .load(&mut establish_connection())
            .unwrap(),
    )
}

#[get("/ticket/remove/<id>")]
fn remove_ticket_by_id(id: i32) -> Option<Json<Ticket>> {
    let conn = &mut establish_connection();
    let tik = tickets::table
        .filter(tickets::id.eq(id))
        .select(Ticket::as_select())
        .get_result(conn);
    let del = diesel::delete(tickets::table)
        .filter(tickets::id.eq(id))
        .execute(conn);
    if del.is_err() || tik.is_err() {
        None
    } else {
        Some(Json(tik.unwrap()))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                new_user,
                get_user_by_id,
                get_user_by_name,
                list_users,
                remove_user_by_id,
                new_ticket,
                get_ticket_by_id,
                get_tickets_by_author_id,
                list_tickets,
                remove_ticket_by_id,
            ],
        )
        .register("/", catchers![default_catcher])
        .attach(Template::fairing())
}
