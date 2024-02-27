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
  pub topics: Vec<(Topic, bool)>,
}

impl Subscriber {
  pub fn new(name: &str, topics: Vec<Topic>) -> Subscriber {
    Subscriber {
      name: name.to_string(),
      id: Uuid::new_v4().to_string(),
      topics: topics.iter().map(|t| (*t, false)).collect(),
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
  pub fn trigger_update(&mut self, topics: Vec<Topic>) {
    self.subscribers = self
      .subscribers
      .iter()
      .map(|s| {
        let mut sbs = s.clone();
        for tp in topics.iter() {
          sbs.topics = sbs
            .topics
            .iter()
            .map(|top| {
              let mut t = top.clone();
              if &t.0 == tp {
                t.1 = true;
              }
              t
            })
            .collect();
        }
        println!("Updating subscriber: {:?}", sbs);
        sbs.clone()
      })
      .collect();
    println!("Subscribers: {:?}", self.subscribers);
  }
  pub fn check_subscriber(&mut self, id: &str, topics: Vec<Topic>) -> Vec<Topic> {
    let mut rtrn: Vec<Topic> = Vec::new();
    self.subscribers = self
      .subscribers
      .iter()
      .map(|sbs| {
        let mut s = sbs.clone();
        if s.id == id {
          s.topics = s
            .topics
            .iter()
            .map(|tpc| {
              let mut t = tpc.clone();
              if t.1 && topics.contains(&t.0) {
                rtrn.push(t.0);
                t.1 = false;
              }
              t
            })
            .collect();
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
