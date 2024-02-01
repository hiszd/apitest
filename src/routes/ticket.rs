use diesel::prelude::*;
use rocket::serde::json::from_str;

use crate::establish_connection;
use crate::model::*;
use crate::schema::*;
use crate::types::json::user::UserJson;
use crate::types::{json::ticket::*, statustype::*, tickettype::*};
use rocket::serde::json::Json;

#[post("/ticket/new", data = "<ticket>")]
pub fn new_ticket(ticket: Json<NewTicketJson>) -> Json<Ticket> {
    // create table entry to tickets for the new ticket
    let tik = diesel::insert_into(tickets::table)
        .values((
            tickets::count.eq(from_str::<i32>(&ticket.count.as_str()).expect("invalid count")),
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
            tickets_authors::author_id
                .eq(from_str::<i32>(&ticket.author_id.as_str()).expect("invalid count")),
            tickets_authors::ticket_id.eq(tik.id),
        ))
        .execute(&mut establish_connection())
        .unwrap();
    Json(tik)
}

#[get("/ticket/<id>")]
pub fn get_ticket_by_id(id: i32) -> Json<Ticket> {
    Json(
        tickets::table
            .filter(tickets::id.eq(id))
            .select(Ticket::as_select())
            .get_result(&mut establish_connection())
            .unwrap(),
    )
}

#[get("/ticket/list/<author_id>")]
pub fn get_tickets_by_author_id(author_id: i32) -> Json<Vec<Ticket>> {
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
pub fn list_tickets() -> Json<Vec<TicketWAuthorJson>> {
    let users = users::table
        .select(User::as_select())
        .load(&mut establish_connection())
        .unwrap();
    let tickets: Vec<TicketWAuthorJson> = users.iter().fold(Vec::new(), |mut acc, user| {
        let tickets = tickets_authors::table
            .filter(tickets_authors::author_id.eq(user.id))
            .inner_join(tickets::table)
            .select(Ticket::as_select())
            .load(&mut establish_connection());
        if tickets.is_err() {
            acc
        } else {
            let tickets_w_author: Vec<TicketWAuthorJson> =
                tickets.unwrap().iter().fold(Vec::new(), |mut acc, ticket| {
                    let tik = TicketWAuthorJson {
                        ticket: Some(TicketJson::from(ticket)),
                        author: Some(UserJson::from(user)),
                    };
                    acc.push(tik);
                    acc
                });
            acc.extend(tickets_w_author);
            acc
        }
    });
    Json(tickets)
}

#[get("/ticket/remove/<id>")]
pub fn remove_ticket_by_id(id: i32) -> Option<Json<Ticket>> {
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
