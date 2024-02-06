use crate::establish_connection;
use crate::model::*;
use crate::schema::*;
use crate::types::json::user::UserJson;
use crate::types::{json::ticket::*, statustype::*, tickettype::*};
use diesel::prelude::*;
use rocket::response::status::NoContent;
use rocket::serde::json::from_str;
use rocket::serde::json::Json;

fn delete_ticket(
    ticket_id: i32,
    conn: &mut PgConnection,
) -> Result<Json<Ticket>, diesel::result::Error> {
    println!("Deleting ticket {}", ticket_id);
    let tik = tickets::table
        .filter(tickets::id.eq(ticket_id))
        .select(Ticket::as_select())
        .get_result(conn);
    let del_auth_rel = diesel::delete(tickets_authors::table)
        .filter(tickets_authors::ticket_id.eq(ticket_id))
        .execute(conn);
    let del_ticket = diesel::delete(tickets::table)
        .filter(tickets::id.eq(ticket_id))
        .execute(conn);

    if tik.is_err() {
        println!("Ticket not found");
        Err(tik.err().unwrap())
    } else if del_auth_rel.is_err() {
        println!("Ticket relation not removed");
        Err(del_auth_rel.err().unwrap())
    } else if del_ticket.is_err() {
        println!("Ticket not removed");
        Err(del_ticket.err().unwrap())
    } else {
        Ok(Json(tik.unwrap()))
    }
}

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

#[options("/ticket/list/all")]
pub fn list_tickets_options() -> NoContent {
    NoContent
}

#[get("/ticket/list/all")]
pub fn list_tickets<'r>() -> String {
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
                        id: ticket.id,
                        count: ticket.count,
                        subject: ticket.subject.clone(),
                        description: ticket.description.clone(),
                        ticktype: ticket.ticktype.to_string(),
                        status: ticket.status.to_string(),
                        author: Some(UserJson::from(user)),
                    };
                    acc.push(tik);
                    acc
                });
            acc.extend(tickets_w_author);
            acc
        }
    });
    rocket::serde::json::serde_json::to_string(&tickets).unwrap()
    // Response::build()
    //     .header(ContentType::JSON)
    //     .raw_header("Cache-Control", "max-age=120")
    //     .sized_body(json.len(), Cursor::new(json))
    //     .finalize()
}

#[get("/ticket/remove/<id>")]
pub fn remove_ticket_by_id(id: i32) -> Option<Json<Ticket>> {
    let conn = &mut establish_connection();
    match delete_ticket(id, conn) {
        Ok(tik) => Some(tik),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}
