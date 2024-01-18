use crate::types::{statustype::StatusType, tickettype::TicketType};
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

use crate::schema::{tickets, tickets_authors, users};

#[derive(
    Selectable, Insertable, Queryable, Serialize, Deserialize, Identifiable, Clone, PartialEq, Debug,
)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(
    Selectable, Insertable, Queryable, Serialize, Deserialize, Identifiable, Clone, PartialEq, Debug,
)]
#[diesel(table_name = tickets)]
pub struct Ticket {
    pub id: i32,
    pub count: i32,
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
