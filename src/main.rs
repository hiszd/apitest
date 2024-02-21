mod db;

use std::env;

use apitest::routes::*;

#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Status};
use rocket::response::status;
use rocket::{Request, Response};
use rocket_dyn_templates::Template;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|_| "*".to_string()),
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})\n{:?}", status, req.uri(), req.headers());
    status::Custom(status, msg)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                user::update_user,
                user::update_user_preflight,
                user::new_user,
                user::new_user_preflight,
                user::new_user_test,
                user::get_user,
                user::get_user_preflight,
                user::list_users,
                user::list_users_preflight,
                user::remove_user,
                user::remove_user_preflight,
                ticket::update_ticket,
                ticket::update_ticket_preflight,
                ticket::new_ticket,
                // ticket::new_ticket_preflight,
                ticket::get_tickets_by_author,
                ticket::list_tickets,
                ticket::list_tickets_preflight,
                ticket::remove_ticket_by_id,
            ],
        )
        .attach(CORS)
        .register("/", catchers![default_catcher])
        .attach(CORS)
}
