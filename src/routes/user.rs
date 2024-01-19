use diesel::prelude::*;

use crate::establish_connection;
use crate::model::*;
use crate::schema::*;
use crate::types::json::*;
use rocket::serde::json::Json;

#[post("/user/new", data = "<user>")]
pub fn new_user(user: Json<NewUserJson>) -> Json<User> {
    let usr = diesel::insert_into(users::table)
        .values(users::name.eq(&user.name))
        .returning(User::as_returning())
        .get_result(&mut establish_connection());
    Json(usr.unwrap())
}

#[get("/user/<id>")]
pub fn get_user_by_id(id: i32) -> Json<User> {
    Json(
        users::table
            .filter(users::id.eq(id))
            .select(User::as_select())
            .get_result(&mut establish_connection())
            .unwrap(),
    )
}

#[get("/user/<name>", rank = 2)]
pub fn get_user_by_name(name: &str) -> Json<User> {
    Json(
        users::table
            .filter(users::name.eq(name))
            .select(User::as_select())
            .get_result(&mut establish_connection())
            .unwrap(),
    )
}

#[get("/user/list/all")]
pub fn list_users() -> Json<Vec<User>> {
    Json(
        users::table
            .select(User::as_select())
            .load(&mut establish_connection())
            .unwrap(),
    )
}

#[get("/user/remove/<id>")]
pub fn remove_user_by_id(id: i32) -> Option<Json<User>> {
    let conn = &mut establish_connection();
    let usr = users::table
        .filter(users::id.eq(id))
        .select(User::as_select())
        .get_result(conn);
    let del = diesel::delete(users::table)
        .filter(users::id.eq(id))
        .execute(conn);
    if del.is_err() || usr.is_err() {
        None
    } else {
        Some(Json(usr.unwrap()))
    }
}
