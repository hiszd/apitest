use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::tokio::sync::Mutex;
use uuid::Uuid;

extern crate tera;
#[macro_use]
extern crate rocket;

pub const SECRET: &str = "y8v86wOCAZ0v5Y+uLwQaMBoLa5HDtEShew7yLwNRNls=";

pub mod model;
pub mod routes;
pub mod schema;
pub mod types;

pub static STATE: Mutex<CustState> = Mutex::const_new(CustState::const_new());

// TODO: Work here!!!!
// pub fn str_from_id(id: String) -> String {
//   return;
// }

#[derive(PartialEq, Debug, Clone)]
pub struct Update {
  pub topic: Topic,
  pub ids: Vec<String>,
}

impl From<Update> for String {
  fn from(t: Update) -> Self {
    return String::from(t.topic) + ":" + &t.ids.join(",");
  }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Topic {
  Users,
  Tickets,
}

impl From<Topic> for String {
  fn from(t: Topic) -> Self {
    match t {
      Topic::Users => "users".to_string(),
      Topic::Tickets => "tickets".to_string(),
    }
  }
}
impl From<Topic> for &str {
  fn from(t: Topic) -> Self {
    match t {
      Topic::Users => "users",
      Topic::Tickets => "tickets",
    }
  }
}
impl TryFrom<String> for Topic {
  type Error = ();
  fn try_from(s: String) -> Result<Self, Self::Error> {
    match s.as_str() {
      "users" => Ok(Topic::Users),
      "Users" => Ok(Topic::Users),
      "tickets" => Ok(Topic::Tickets),
      "Tickets" => Ok(Topic::Tickets),
      _ => Err(()),
    }
  }
}
impl TryFrom<&str> for Topic {
  type Error = ();
  fn try_from(s: &str) -> Result<Self, Self::Error> {
    match s {
      "users" => Ok(Topic::Users),
      "Users" => Ok(Topic::Users),
      "tickets" => Ok(Topic::Tickets),
      "Tickets" => Ok(Topic::Tickets),
      _ => Err(()),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Subscriber {
  pub name: String,
  pub id: String,
  pub topics: Vec<Topic>,
  pub updates: Vec<Update>,
}

impl Subscriber {
  pub fn new(name: &str, topics: Vec<Topic>) -> Subscriber {
    Subscriber {
      name: name.to_string(),
      id: Uuid::new_v4().to_string(),
      topics: topics.clone(),
      updates: Vec::new(),
    }
  }
}

pub struct CustState {
  subscribers: Vec<Subscriber>,
}

impl CustState {
  pub fn new() -> CustState {
    CustState {
      subscribers: Vec::new(),
    }
  }
  pub const fn const_new() -> CustState {
    CustState {
      subscribers: Vec::new(),
    }
  }
  pub fn subscribe(&mut self, name: &str, topics: Vec<Topic>) -> String {
    let sub = Subscriber::new(name, topics);
    println!("Creating subscriber: {:?}", sub);
    self.subscribers.push(sub.clone());
    sub.id
  }
  pub fn unsubscribe(&mut self, id: &str) {
    self.subscribers.retain(|s| s.id != id);
  }
  pub fn trigger_update(&mut self, update: Update) {
    self.subscribers = self
      .subscribers
      .iter()
      .map(|s| {
        let mut sbs = s.clone();
        if sbs.topics.contains(&update.topic) {
          sbs.updates.push(update.clone());
        }
        println!("Updating subscriber: {:?}", sbs);
        sbs.clone()
      })
      .collect();
    println!("Subscribers: {:?}", self.subscribers);
  }
  pub fn check_subscriber(&mut self, id: &str, topic: Topic) -> Vec<Update> {
    let mut rtrn: Vec<Update> = Vec::new();
    self.subscribers = self
      .subscribers
      .iter()
      .map(|sbs| {
        let mut s = sbs.clone();
        if s.id == id {
          if s.updates.len() > 0 && s.topics.contains(&topic) {
            rtrn.append(&mut s.updates.clone());
            s.updates = Vec::new();
          }
        }
        s.clone()
      })
      .collect();
    rtrn
  }
}

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
