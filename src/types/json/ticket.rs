use crate::model::*;
use rocket::serde::{Deserialize, Serialize};

use super::shared::SecretData;
use super::user::UserJson;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TicketSelectJson {
    pub id: Option<i32>,
    pub author_id: Option<i32>,
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
    pub subject: String,
    pub description: String,
    pub ticktype: String,
    pub status: String,
}
impl SecretData for NewTicketJson {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TicketWAuthorJson {
    pub id: i32,
    pub subject: String,
    pub description: String,
    pub ticktype: String,
    pub status: String,
    pub author: Option<UserJson>,
}
impl SecretData for TicketWAuthorJson {}

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
