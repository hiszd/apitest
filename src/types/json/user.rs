use crate::model::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserSelectJson {
    pub secret: String,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub email: Option<String>,
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
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl From<&User> for UserJson {
    fn from(u: &User) -> Self {
        UserJson {
            id: u.id,
            name: u.name.clone(),
            email: u.email.clone(),
        }
    }
}

impl From<User> for UserJson {
    fn from(u: User) -> Self {
        UserJson {
            id: u.id,
            name: u.name,
            email: u.email,
        }
    }
}
