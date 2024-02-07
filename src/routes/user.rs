use diesel::prelude::*;
use rocket::response::status::NoContent;

use crate::establish_connection;
use crate::model::*;
use crate::schema::*;
use crate::types::json::user::*;
use rocket::serde::json::Json;

#[get("/user/new/test")]
pub fn new_user_test() -> Json<User> {
    let newuser = NewUser {
        name: "test".to_string(),
        email: "test@test.com".to_string(),
    };
    let usr = diesel::insert_into(users::table)
        .values(&newuser)
        .returning(User::as_returning())
        .get_result(&mut establish_connection());
    Json(usr.unwrap())
}

#[post("/user/new", data = "<user>")]
pub fn new_user(user: Json<NewUserJson>) -> Json<User> {
    assert!(
        user.secret == crate::SECRET,
        "Wrong secret: {}, {}",
        user.secret,
        crate::SECRET
    );

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

#[post("/user/get", data = "<data>")]
pub fn get_user(data: Json<UserSelectJson>) -> Result<Json<User>, ()> {
    println!("{:?}", data);
    if data.secret != crate::SECRET {
        println!("Wrong secret: {}, {}", data.secret, crate::SECRET);
        return Err(());
    }
    let mut fltr = users::table.into_boxed();
    if let Some(id) = data.id {
        fltr = fltr.filter(users::id.eq(id));
    }
    if let Some(name) = &data.name {
        fltr = fltr.filter(users::name.eq(name));
    }
    if let Some(email) = &data.email {
        fltr = fltr.filter(users::email.eq(email));
    }
    Ok(Json(
        fltr.first(&mut establish_connection())
            .expect("Error loading users"),
    ))
}

#[options("/user/get")]
pub fn get_user_preflight() -> NoContent {
    NoContent
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

#[post("/user/remove", data = "<data>")]
pub fn remove_user(data: Json<UserSelectJson>) -> Result<Json<User>, ()> {
    println!("{:?}", data);

    let conn = &mut establish_connection();

    if data.secret != crate::SECRET {
        println!("Wrong secret: {}, {}", data.secret, crate::SECRET);
        return Err(());
    }

    let mut fltr = users::table.into_boxed();

    if let Some(id) = data.id {
        fltr = fltr.filter(users::id.eq(id));
    }
    if let Some(name) = &data.name {
        fltr = fltr.filter(users::name.eq(name));
    }
    if let Some(email) = &data.email {
        fltr = fltr.filter(users::email.eq(email));
    }

    let usr: User = fltr.first(conn).expect("Error loading users");

    diesel::delete(users::table)
        .filter(users::id.eq(usr.id))
        .execute(conn)
        .expect("Error deleting user");

    Ok(Json(usr))
}
