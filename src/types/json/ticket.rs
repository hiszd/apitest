use crate::model::*;
use rocket::serde::{Deserialize, Serialize};

use super::user::UserJson;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GetTicketJson {
    pub secret: String,
    pub author_id: Option<String>,
    pub count: Option<String>,
    pub subject: Option<String>,
    pub description: Option<String>,
    pub ticktype: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewTicketJson {
    pub secret: String,
    pub author_id: String,
    pub count: String,
    pub subject: String,
    pub description: String,
    pub ticktype: String,
    pub status: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TicketWAuthorJson {
    pub ticket: Option<TicketJson>,
    pub author: Option<UserJson>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TicketJson {
    pub id: i32,
    pub count: i32,
    pub subject: String,
    pub description: String,
    pub ticktype: String,
    pub status: String,
}

impl From<&Ticket> for TicketJson {
    fn from(t: &Ticket) -> Self {
        TicketJson {
            id: t.id,
            count: t.count,
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
            count: t.count,
            subject: t.subject,
            description: t.description,
            ticktype: t.ticktype.to_string(),
            status: t.status.to_string(),
        }
    }
}
