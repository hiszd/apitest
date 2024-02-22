use crate::establish_connection;
use crate::model::*;
use crate::schema::*;
use crate::types::json::shared::WithSecret;
use crate::types::json::user::UserJson;
use crate::types::json::user::UserSelectJson;
use crate::types::{json::ticket::*, statustype::*, tickettype::*};
use diesel::prelude::*;
use rocket::response::status::NoContent;
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

#[options("/ticket/new")]
pub fn new_ticket_preflight() -> NoContent {
    NoContent
}

#[post("/ticket/new", data = "<ticket>")]
pub fn new_ticket(ticket: Json<WithSecret<NewTicketJson>>) -> Json<Ticket> {
    // create table entry to tickets for the new ticket
    let tik = diesel::insert_into(tickets::table)
        .values((
            tickets::count.eq(&ticket.data.count),
            tickets::subject.eq(&ticket.data.subject),
            tickets::description.eq(&ticket.data.description),
            tickets::status.eq(StatusType::from(ticket.data.status.clone())),
            tickets::ticktype.eq(TicketType::from(ticket.data.ticktype.clone())),
        ))
        .returning(Ticket::as_returning())
        .get_result(&mut establish_connection())
        .unwrap();

    // create table entry to tickets_authors for the relation between the ticket and the author
    diesel::insert_into(tickets_authors::table)
        .values((
            tickets_authors::author_id.eq(&ticket.data.author_id),
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

#[options("/ticket/list/author")]
pub fn get_tickets_by_author_preflight() -> NoContent {
    NoContent
}

#[post("/ticket/list/author", data = "<data>")]
pub fn get_tickets_by_author(
    data: Json<WithSecret<UserSelectJson>>,
) -> Result<Json<Vec<Ticket>>, ()> {
    println!("{:?}", data);
    if data.secret != crate::SECRET {
        println!("Wrong secret: {}, {}", data.secret, crate::SECRET);
        return Err(());
    }
    let mut fltr = users::table.into_boxed();
    if let Some(f) = data.data.id {
        fltr = fltr.filter(users::id.eq(f));
    }
    if let Some(f) = &data.data.name {
        fltr = fltr.filter(users::name.eq(f));
    }
    if let Some(f) = &data.data.email {
        fltr = fltr.filter(users::email.eq(f));
    }
    let user = fltr.first::<User>(&mut establish_connection());

    match user {
        Ok(u) => {
            let rslt = tickets_authors::table
                .inner_join(tickets::table)
                .filter(tickets_authors::author_id.eq(u.id))
                .select(Ticket::as_select())
                .load(&mut establish_connection());
            match rslt {
                Ok(tik) => Ok(Json(tik)),
                Err(_) => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}

#[options("/ticket/update")]
pub fn update_ticket_preflight() -> NoContent {
    NoContent
}

#[post("/ticket/update", data = "<data>")]
pub fn update_ticket(data: Json<WithSecret<TicketWAuthorJson>>) -> Result<Json<Ticket>, ()> {
    if data.secret != crate::SECRET {
        println!("Wrong secret: {}, {}", data.secret, crate::SECRET);
        return Err(());
    }

    println!("Updating ticket {:?}", data);

    // let rslt = tickets::table
    //     .filter(tickets::id.eq(data.data.id))
    //     .get_result(&mut establish_connection());

    let rslt = diesel::update(tickets::table.filter(tickets::id.eq(data.data.id)))
        .set((
            tickets::subject.eq(data.data.subject.clone()),
            tickets::description.eq(data.data.description.clone()),
            tickets::status.eq(StatusType::from(data.data.status.clone())),
            tickets::ticktype.eq(TicketType::from(data.data.ticktype.clone())),
        ))
        .get_result(&mut establish_connection());

    if data.data.author.is_some() {
        diesel::update(tickets_authors::table)
            .filter(tickets_authors::ticket_id.eq(data.data.id))
            .set(tickets_authors::author_id.eq(data.data.author.as_ref().unwrap().id))
            .execute(&mut establish_connection())
            .unwrap();
    }

    match rslt {
        Ok(tik) => {
            println!("Updated ticket {:?}", tik);
            Ok(Json(tik))
        }
        Err(_) => Err(()),
    }
}

#[options("/ticket/get")]
pub fn get_ticket_preflight() -> NoContent {
    NoContent
}

#[post("/ticket/get", data = "<data>")]
pub fn get_ticket(data: Json<WithSecret<TicketSelectJson>>) -> Result<Json<Ticket>, ()> {
    println!("{:?}", data);
    if data.secret != crate::SECRET {
        println!("Wrong secret: {}, {}", data.secret, crate::SECRET);
        return Err(());
    }
    let mut fltr = tickets::table.into_boxed();
    if let Some(id) = data.data.id {
        fltr = fltr.filter(tickets::id.eq(id));
    }
    if let Some(subject) = &data.data.subject {
        fltr = fltr.filter(tickets::subject.eq(subject));
    }
    if let Some(description) = &data.data.description {
        fltr = fltr.filter(tickets::description.eq(description));
    }
    if let Some(status) = &data.data.status {
        fltr = fltr.filter(tickets::status.eq(StatusType::from(status.clone())));
    }
    if let Some(ticktype) = &data.data.ticktype {
        fltr = fltr.filter(tickets::ticktype.eq(TicketType::from(ticktype.clone())));
    }
    let rslt = fltr.first(&mut establish_connection());
    match rslt {
        Ok(tik) => Ok(Json(tik)),
        Err(_) => Err(()),
    }
}

#[options("/ticket/list/all")]
pub fn list_tickets_preflight() -> NoContent {
    NoContent
}

// TODO: when there are no users that have tickets assigned, then no tickets are returned.
// this is very wrong. WORK on this!!
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
    println!("TICKETS: {:?}", tickets);
    rocket::serde::json::serde_json::to_string(&tickets).unwrap()
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
