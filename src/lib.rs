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
