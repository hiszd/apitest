use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use uuid::Uuid;

extern crate tera;
#[macro_use]
extern crate rocket;

pub const SECRET: &str = "y8v86wOCAZ0v5Y+uLwQaMBoLa5HDtEShew7yLwNRNls=";

pub mod model;
pub mod routes;
pub mod schema;
pub mod types;

#[derive(PartialEq, Debug, Clone)]
pub enum Topic {
  Users,
  Tickets,
}

// impl From<String> for Topic {
//   fn from(s: String) -> Self {
//     match s {
//       String::from("users") => Topic::Users,
//       String::from("Users") => Topic::Users,
//       String::from("tickets") => Topic::Tickets,
//       String::from("Tickets") => Topic::Tickets,
//       _ => Topic::Users,
//     }
//   }
// }

// impl From<&str> for Topic {
//   fn from(s: &str) -> Self {
//     match s {
//       "users" => Topic::Users,
//       "Users" => Topic::Users,
//       "tickets" => Topic::Tickets,
//       "Tickets" => Topic::Tickets,
//       _ => Topic::Users,
//     }
//   }
// }

pub struct Subscriber {
  pub name: String,
  pub id: String,
  pub topic: Topic,
  pub needs_update: bool,
}

impl Subscriber {
  pub fn new(name: &str, topic: Topic) -> Subscriber {
    Subscriber {
      name: name.to_string(),
      id: Uuid::new_v4().to_string(),
      needs_update: false,
      topic,
    }
  }
}

pub struct CustState {
  users_update: bool,
  tickets_update: bool,
  subscribers: Vec<Subscriber>,
}

impl CustState {
  pub fn new() -> CustState {
    CustState {
      users_update: false,
      tickets_update: false,
      subscribers: Vec::new(),
    }
  }
  pub const fn const_new() -> CustState {
    CustState {
      users_update: false,
      tickets_update: false,
      subscribers: Vec::new(),
    }
  }
  pub fn subscribe(&mut self, name: &str, topic: Topic) -> &str {
    let sub = Subscriber::new(name, topic);
    self.subscribers.push(sub);
    sub.id.as_str()
  }
  pub fn unsubscribe(&mut self, id: &str) {
    self.subscribers.retain(|s| s.id != id);
  }
  pub fn trigger_update(&mut self, topic: Topic) {
    self.subscribers.iter_mut().map(|s| {
      if s.topic == topic {
        s.needs_update = true
      }
    });
  }
  pub fn check_update(&mut self, id: &str) -> bool {
    let mut rtrn = false;
    self
      .subscribers
      .iter_mut()
      .find(|s| s.id == id)
      .map(|s| {
        s.needs_update = false;
        rtrn = s.needs_update
      })
      .unwrap();
    rtrn
  }
}

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
