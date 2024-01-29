use diesel::prelude::*;

use crate::establish_connection;
use crate::model::*;
use crate::schema::*;
use crate::types::json::user::*;
use rocket::serde::json::Json;

#[post("/user/new", data = "<user>")]
pub fn new_user(user: Json<NewUserJson>) -> Json<User> {
    let newuser = NewUser {
        name: user.name.clone(),
        email: user.email.clone(),
    };
    let usr = diesel::insert_into(users::table)
        .values(&newuser)
        .returning(User::as_returning())
        .get_result(&mut establish_connection());
    Json(usr.unwrap())
}

// WORK HERE
// TODO: change all requests to POST and confirm secret before doing anything
#[post("/user/get", data = "<data>")]
pub fn get_user(data: Json<GetUserJson>) -> Result<Json<User>, ()> {
    println!("{:?}", data);

    if data.secret != crate::SECRET {
        println!("Wrong secret: {}, {}", data.secret, crate::SECRET);
        return Err(());
    }

    let mut fltr = users::table.into_boxed();

    if data.id.is_some() {
        fltr = fltr.filter(users::id.eq(data.id.unwrap()));
    }
    if data.name.is_some() {
        fltr = fltr.filter(users::name.eq(data.name.unwrap()));
    }
    if data.email.is_some() {
        fltr = fltr.filter(users::email.eq(data.email.unwrap()));
    }

    Ok(Json(
        // https://stackoverflow.com/questions/65039754/rust-diesel-conditionally-filter-a-query
        // https://docs.rs/diesel/1.4.8/diesel/query_dsl/trait.QueryDsl.html#method.into_boxed
        fltr.load(&mut establish_connection())
            .expect("Error loading users")[0],
    ))
}

// TODO: change all requests to POST and confirm secret before doing anything
#[get("/user/<email>", rank = 2)]
pub fn get_user_by_email(email: &str) -> Json<User> {
    Json(
        users::table
            .filter(users::name.eq(email))
            .select(User::as_select())
            .get_result(&mut establish_connection())
            .unwrap(),
    )
}

// TODO: change all requests to POST and confirm secret before doing anything
#[get("/user/list/all")]
pub fn list_users() -> Json<Vec<User>> {
    Json(
        users::table
            .select(User::as_select())
            .load(&mut establish_connection())
            .unwrap(),
    )
}

// TODO: change all requests to POST and confirm secret before doing anything
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
