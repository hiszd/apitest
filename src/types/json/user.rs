use crate::model::*;
use rocket::serde::{Deserialize, Serialize};

use super::shared::SecretData;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserSelectJson {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub email: Option<String>,
}
impl SecretData for UserSelectJson {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewUserJson {
    pub name: String,
    pub email: String,
}
impl SecretData for NewUserJson {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserJson {
    pub id: i32,
    pub name: String,
    pub email: String,
}
impl SecretData for UserJson {}

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
