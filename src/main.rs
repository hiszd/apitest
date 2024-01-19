mod db;

use apitest::routes::*;

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::status;
use rocket::Request;
use rocket_dyn_templates::Template;

#[catch(default)]
fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
    let msg = format!("{} ({})\n{:?}", status, req.uri(), req.headers());
    status::Custom(status, msg)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                user::new_user,
                user::get_user_by_id,
                user::get_user_by_name,
                user::list_users,
                user::remove_user_by_id,
                ticket::new_ticket,
                ticket::get_ticket_by_id,
                ticket::get_tickets_by_author_id,
                ticket::list_tickets,
                ticket::remove_ticket_by_id,
            ],
        )
        .register("/", catchers![default_catcher])
        .attach(Template::fairing())
}
