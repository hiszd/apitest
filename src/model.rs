use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

use crate::schema::{tickets, tickets_agents, tickets_authors, users};
use crate::types::{statustype::StatusType, tickettype::TicketType};

#[derive(Selectable, Queryable, Serialize, Deserialize, Identifiable, Clone, PartialEq, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
  pub name: String,
  pub email: String,
}

#[derive(Selectable, Queryable, Serialize, Deserialize, Identifiable, Clone, PartialEq, Debug)]
#[diesel(table_name = tickets)]
pub struct Ticket {
  pub id: i32,
  pub subject: String,
  pub description: String,
  pub status: StatusType,
  pub ticktype: TicketType,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = tickets)]
pub struct NewTicket {
  pub subject: String,
  pub description: String,
  pub status: StatusType,
  pub ticktype: TicketType,
}

#[derive(
  Selectable, Insertable, Queryable, Serialize, Deserialize, Identifiable, Clone, PartialEq, Debug,
)]
#[diesel(belongs_to(Ticket))]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(table_name = tickets_authors)]
#[diesel(primary_key(ticket_id, author_id))]
pub struct TicketAuthor {
  pub ticket_id: i32,
  pub author_id: i32,
}

#[derive(
  Selectable, Insertable, Queryable, Serialize, Deserialize, Identifiable, Clone, PartialEq, Debug,
)]
#[diesel(belongs_to(Ticket))]
#[diesel(belongs_to(User, foreign_key = agent_id))]
#[diesel(table_name = tickets_agents)]
#[diesel(primary_key(ticket_id, agent_id))]
pub struct TicketAgent {
  pub ticket_id: i32,
  pub agent_id: i32,
}
