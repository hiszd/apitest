use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::serde::{Deserialize, Serialize};

use super::shared::SecretData;
use super::user::UserJson;
use crate::schema::{tickets, tickets_agents, tickets_authors, users};
use crate::{establish_connection, model::*};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TicketSelectJson {
  pub id: Option<i32>,
  pub author_id: Option<i32>,
  pub agent_id: Option<i32>,
  pub subject: Option<String>,
  pub description: Option<String>,
  pub ticktype: Option<String>,
  pub status: Option<String>,
}
impl SecretData for TicketSelectJson {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewTicketJson {
  pub author_id: i32,
  pub agent_id: Option<i32>,
  pub subject: String,
  pub description: String,
  pub ticktype: String,
  pub status: String,
}
impl SecretData for NewTicketJson {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TicketWithAuthorJson {
  pub id: i32,
  pub subject: String,
  pub description: String,
  pub ticktype: String,
  pub status: String,
  pub author: UserJson,
}
impl SecretData for TicketWithAuthorJson {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TicketWithUsersJson {
  pub id: i32,
  pub subject: String,
  pub description: String,
  pub ticktype: String,
  pub status: String,
  pub author: UserJson,
  pub agent: Option<UserJson>,
}
impl SecretData for TicketWithUsersJson {}

impl TicketWithUsersJson {
  pub fn from_ticket(t: &Ticket, connie: Option<&mut PgConnection>) -> Result<Self, ()> {
    if connie.is_none() {
      let mut conn = establish_connection();
      return Self::from_ticket(t, Some(&mut conn));
    }
    let conn = connie.unwrap();

    let ticketpre: Result<Ticket, diesel::result::Error> = tickets::table
      .filter(tickets::id.eq(t.id))
      .select(Ticket::as_select())
      .first(conn);
    if ticketpre.is_err() {
      println!("Ticket not found. Error: {:?}", ticketpre.unwrap_err());
      return Err(());
    }
    let ticket = ticketpre.unwrap();
    let authorpre = users::table
      .inner_join(tickets_authors::table)
      .filter(tickets_authors::ticket_id.eq(t.id))
      .select(User::as_select())
      .first(conn);
    if authorpre.is_err() {
      println!("Author not found. Error: {:?}", authorpre.unwrap_err());
      return Err(());
    }
    let author = UserJson::from(authorpre.unwrap());
    let mut agent: Option<UserJson> = None;
    let agentpre = users::table
      .inner_join(tickets_agents::table)
      .filter(tickets_agents::ticket_id.eq(t.id))
      .select(User::as_select())
      .first(conn);
    if !agentpre.is_err() {
      agent = Some(UserJson::from(agentpre.unwrap()));
    }
    Ok(TicketWithUsersJson {
      id: ticket.id,
      subject: ticket.subject,
      description: ticket.description,
      ticktype: ticket.ticktype.to_string(),
      status: ticket.status.to_string(),
      author,
      agent,
    })
  }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TicketJson {
  pub id: i32,
  pub subject: String,
  pub description: String,
  pub ticktype: String,
  pub status: String,
}
impl SecretData for TicketJson {}

impl From<&Ticket> for TicketJson {
  fn from(t: &Ticket) -> Self {
    TicketJson {
      id: t.id,
      subject: t.subject.clone(),
      description: t.description.clone(),
      ticktype: t.ticktype.to_string(),
      status: t.status.to_string(),
    }
  }
}

impl From<Ticket> for TicketJson {
  fn from(t: Ticket) -> Self {
    TicketJson {
      id: t.id,
      subject: t.subject,
      description: t.description,
      ticktype: t.ticktype.to_string(),
      status: t.status.to_string(),
    }
  }
}
