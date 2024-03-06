use diesel::prelude::*;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use crate::establish_connection;
use crate::model::*;
use crate::schema::*;
use crate::types::json::shared::JustSecret;
use crate::types::json::shared::WithSecret;
use crate::types::json::user::UserJson;
use crate::types::json::user::UserSelectJson;
use crate::types::{json::ticket::*, statustype::*, tickettype::*};
use crate::Topic;
use crate::STATE;

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
pub async fn new_ticket(
  ticket: Json<WithSecret<NewTicketJson>>,
) -> Result<Json<TicketWithUsersJson>, ()> {
  println!("{:?}", ticket);
  if ticket.secret != crate::SECRET {
    println!("Wrong secret: {}, {}", ticket.secret, crate::SECRET);
    return Err(());
  }
  let conn = &mut establish_connection();
  // check to see if author exists
  let athrpre: Result<User, diesel::result::Error> = users::table
    .filter(users::id.eq(&ticket.data.author_id))
    .select(User::as_select())
    .get_result(conn);
  if athrpre.is_err() {
    println!("Author not found");
    return Err(());
  }
  let athr = UserJson::from(athrpre.unwrap());
  // check to see if agent was specified and exists
  let mut agntpre: Option<Result<User, diesel::result::Error>> = None;
  if ticket.data.agent_id.is_some() {
    let agnttmp = users::table
      .filter(users::id.eq(&ticket.data.author_id))
      .select(User::as_select())
      .get_result(conn);
    if agnttmp.is_err() {
      println!("Author not found");
      return Err(());
    }
    agntpre = Some(agnttmp);
  }
  let mut agnt: Option<UserJson> = None;
  if agntpre.is_some() {
    agnt = Some(UserJson::from(agntpre.unwrap().unwrap()));
  }

  // create table entry to tickets for the new ticket
  let tik = diesel::insert_into(tickets::table)
    .values((
      tickets::subject.eq(&ticket.data.subject),
      tickets::description.eq(&ticket.data.description),
      tickets::status.eq(StatusType::from(ticket.data.status.clone())),
      tickets::ticktype.eq(TicketType::from(ticket.data.ticktype.clone())),
    ))
    .returning(Ticket::as_returning())
    .get_result(conn);
  if tik.is_err() {
    println!("Ticket not created");
    return Err(());
  }
  let tunwrap = tik.unwrap();

  // create table entry to tickets_authors for the relation between the ticket and
  // the author
  let ins = diesel::insert_into(tickets_authors::table)
    .values((
      tickets_authors::author_id.eq(&ticket.data.author_id),
      tickets_authors::ticket_id.eq(tunwrap.id),
    ))
    .execute(conn);
  if ins.is_err() {
    println!("Association not created");
    match delete_ticket(tunwrap.id, conn) {
      Ok(_) => {
        println!("Ticket deleted");
        return Err(());
      }
      Err(_) => {
        println!("Ticket NOT deleted");
        return Err(());
      }
    }
  }
  let tik = TicketWithUsersJson {
    id: tunwrap.id,
    subject: tunwrap.subject.clone(),
    description: tunwrap.description.clone(),
    ticktype: tunwrap.ticktype.to_string(),
    status: tunwrap.status.to_string(),
    author: athr,
    agent: agnt,
  };

  STATE.lock().await.trigger_update(vec![Topic::Tickets]);
  Ok(Json(tik))
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
  let user: Result<User, diesel::result::Error> = fltr.first(&mut establish_connection());
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

#[options("/ticket/list/agent")]
pub fn get_tickets_by_agent_preflight() -> NoContent {
  NoContent
}
#[post("/ticket/list/agent", data = "<data>")]
pub fn get_tickets_by_agent(
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
  let user: Result<User, diesel::result::Error> = fltr.first(&mut establish_connection());
  match user {
    Ok(u) => {
      let rslt = tickets_agents::table
        .inner_join(tickets::table)
        .filter(tickets_agents::agent_id.eq(u.id))
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
pub async fn update_ticket(
  data: Json<WithSecret<TicketWithUsersJson>>,
) -> Result<Json<Ticket>, ()> {
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

  diesel::update(tickets_authors::table)
    .filter(tickets_authors::ticket_id.eq(data.data.id))
    .set(tickets_authors::author_id.eq(data.data.author.id))
    .execute(&mut establish_connection())
    .unwrap();

  match rslt {
    Ok(tik) => {
      STATE.lock().await.trigger_update(vec![Topic::Tickets]);
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
pub fn get_ticket(
  data: Json<WithSecret<TicketSelectJson>>,
) -> Result<Json<TicketWithUsersJson>, ()> {
  println!("{:?}", data);
  if data.secret != crate::SECRET {
    println!("Wrong secret: {}, {}", data.secret, crate::SECRET);
    return Err(());
  }
  let conn = &mut establish_connection();
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
  let rslt = fltr.first(conn);
  if rslt.is_err() {
    println!("Ticket not found. Error: {:?}", rslt.unwrap_err());
    return Err(());
  }

  Ok(Json(
    TicketWithUsersJson::from_ticket(&rslt.unwrap(), Some(conn)).unwrap(),
  ))
}

#[options("/ticket/list/all")]
pub fn list_tickets_preflight() -> NoContent {
  NoContent
}
// TODO: when there are no users that have tickets assigned, then no tickets are
// returned. this is very wrong. WORK on this!!
#[post("/ticket/list/all", data = "<data>")]
pub fn list_tickets<'r>(data: Json<JustSecret>) -> Result<Json<Vec<TicketWithUsersJson>>, ()> {
  if data.secret != crate::SECRET {
    println!("Wrong secret: {}, {}", data.secret, crate::SECRET);
    return Err(());
  }
  let conn = &mut establish_connection();
  let users = users::table.select(User::as_select()).load(conn).unwrap();
  let tickets: Vec<TicketWithUsersJson> = users.iter().fold(Vec::new(), |mut acc, user| {
    let ticketspre = tickets_authors::table
      .filter(tickets_authors::author_id.eq(user.id))
      .inner_join(tickets::table)
      .select(Ticket::as_select())
      .load(conn);
    if ticketspre.is_err() {
      acc
    } else {
      let tickets: Vec<TicketWithUsersJson> =
        ticketspre
          .unwrap()
          .iter()
          .fold(Vec::new(), |mut acc, ticket| {
            acc.push(TicketWithUsersJson::from_ticket(ticket, Some(conn)).unwrap());
            acc
          });
      acc.extend(tickets);
      acc
    }
  });
  println!("TICKETS: {:?}", tickets);
  Ok(Json(tickets))
}

#[options("/ticket/remove")]
pub fn remove_ticket_preflight() -> NoContent {
  NoContent
}
#[post("/ticket/remove", data = "<data>")]
pub async fn remove_ticket(data: Json<WithSecret<TicketSelectJson>>) -> Result<Json<Ticket>, ()> {
  println!("{:?}", data);
  if data.secret != crate::SECRET {
    println!("Wrong secret: {}, {}", data.secret, crate::SECRET);
    return Err(());
  }
  let conn = &mut establish_connection();
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
  let rslt: Result<Ticket, diesel::result::Error> = fltr.first(&mut establish_connection());
  match rslt {
    Ok(tik) => match delete_ticket(tik.id, conn) {
      Ok(tik) => {
        STATE.lock().await.trigger_update(vec![Topic::Tickets]);
        println!("Updated ticket {:?}", tik);
        Ok(tik)
      }
      Err(e) => {
        println!("Error: {}", e);
        Err(())
      }
    },
    Err(_) => Err(()),
  }
}

#[get("/ticket/reset")]
pub async fn reset_tickets() -> NoContent {
  STATE.lock().await.trigger_update(vec![Topic::Tickets]);
  println!("Reset tickets");
  NoContent
}
