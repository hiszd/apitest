use crate::model::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GetUserJson {
    pub secret: String,
    pub id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewUserJson {
    pub secret: String,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserJson {
    pub secret: String,
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl From<&User> for UserJson {
    fn from(u: &User) -> Self {
        UserJson {
            secret: crate::SECRET.to_string(),
            id: u.id,
            name: u.name.clone(),
            email: u.email.clone(),
        }
    }
}

impl From<User> for UserJson {
    fn from(u: User) -> Self {
        UserJson {
            secret: crate::SECRET.to_string(),
            id: u.id,
            name: u.name,
            email: u.email,
        }
    }
}
