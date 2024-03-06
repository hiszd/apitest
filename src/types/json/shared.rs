use serde::{Deserialize, Serialize};

pub trait SecretData {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct WithSecret<T: SecretData> {
  pub secret: String,
  pub data: T,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct JustSecret {
  pub secret: String,
}

impl<T: SecretData> WithSecret<T> {
  pub fn new(secret: String, data: T) -> Self {
    Self { secret, data }
  }
}

impl SecretData for i32 {}
