use crate::model::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewUserJson {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserJson {
    pub id: i32,
    pub name: String,
}

impl From<&User> for UserJson {
    fn from(u: &User) -> Self {
        UserJson {
            id: u.id,
            name: u.name.clone(),
        }
    }
}

impl From<User> for UserJson {
    fn from(u: User) -> Self {
        UserJson {
            id: u.id,
            name: u.name,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewTicketJson {
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
