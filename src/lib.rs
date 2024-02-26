use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::http::Header;
use rocket::Response;
use std::env;

extern crate tera;
#[macro_use]
extern crate rocket;

pub const SECRET: &str = "y8v86wOCAZ0v5Y+uLwQaMBoLa5HDtEShew7yLwNRNls=";

pub mod model;
pub mod routes;
pub mod schema;
pub mod types;

pub struct MyConfig {
    users_update: bool,
    tickets_update: bool,
    subscribers: Vec<Box<dyn FnOnce()>>,
}

impl MyConfig {
    pub fn new() -> MyConfig {
        MyConfig {
            users_update: false,
            tickets_update: false,
            subscribers: Vec::new(),
        }
    }
    pub const fn const_new() -> MyConfig {
        MyConfig {
            users_update: false,
            tickets_update: false,
            subscribers: Vec::new(),
        }
    }
    pub fn subscribe(&mut self, func: Box<dyn FnOnce()>) {
        self.subscribers.push(func);
    }
    pub fn get_users_update(&self) -> bool {
        self.users_update
    }
    pub fn set_users_update(&mut self, value: bool) {
        self.users_update = value;
    }
    pub fn get_tickets_update(&self) -> bool {
        self.tickets_update
    }
    pub fn set_tickets_update(&mut self, value: bool) {
        self.tickets_update = value;
    }
}

pub fn add_headers<'r>(response: &'r mut Response<'r>) -> &'r mut rocket::Response<'r> {
    response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
    response.set_header(Header::new(
        "Access-Control-Allow-Methods",
        "POST, GET, OPTIONS",
    ));
    response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    return response;
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
