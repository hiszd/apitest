use diesel::prelude::*;

use crate::schema::{tickets, tickets_authors, users};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(PartialEq, Debug, diesel_derive_enum::DbEnum, diesel::Expression)]
pub enum TicketType {
    HARDWARE,
    SOFTWARE,
    EMAIL,
    EMPLOYEE,
}

impl From<String> for TicketType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "hardware" => TicketType::HARDWARE,
            "software" => TicketType::SOFTWARE,
            "email" => TicketType::EMAIL,
            "employee" => TicketType::EMPLOYEE,
            _ => TicketType::HARDWARE,
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = tickets)]
pub struct Ticket {
    pub id: i32,
    pub number: i32,
    pub subject: String,
    pub description: String,
    pub ticktype: TicketType,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Ticket))]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(table_name = tickets_authors)]
#[diesel(primary_key(ticket_id, author_id))]
pub struct TicketAuthor {
    pub ticket_id: i32,
    pub author_id: i32,
}
