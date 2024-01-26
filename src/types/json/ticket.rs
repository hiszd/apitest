use crate::model::*;
use rocket::serde::{Deserialize, Serialize};

use super::user::UserJson;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewTicketJson {
    pub secret: String,
    pub author_id: i32,
    pub count: i32,
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
    pub secret: String,
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
            secret: crate::SECRET.to_string(),
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
            secret: crate::SECRET.to_string(),
            id: t.id,
            count: t.count,
            subject: t.subject,
            description: t.description,
            ticktype: t.ticktype.to_string(),
            status: t.status.to_string(),
        }
    }
}
