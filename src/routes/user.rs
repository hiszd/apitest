use diesel::prelude::*;
use rocket::response::status::NoContent;

use crate::establish_connection;
use crate::model::*;
use crate::schema::*;
use crate::types::json::shared::WithSecret;
use crate::types::json::user::*;
use rocket::serde::json::Json;

fn delete_user(user_id: i32, conn: &mut PgConnection) -> Result<Json<User>, diesel::result::Error> {
    println!("Deleting user {}", user_id);
    let usr = users::table
        .filter(users::id.eq(user_id))
        .select(User::as_select())
        .get_result(conn);
    let del_auth_rel = diesel::delete(tickets_authors::table)
        .filter(tickets_authors::author_id.eq(user_id))
        .execute(conn);
    let del_user = diesel::delete(users::table)
        .filter(users::id.eq(user_id))
        .execute(conn);

    if usr.is_err() {
        println!("User not found");
        Err(usr.err().unwrap())
    } else if del_auth_rel.is_err() {
        println!("User relation not removed");
        Err(del_auth_rel.err().unwrap())
    } else if del_user.is_err() {
        println!("User not removed");
        Err(del_user.err().unwrap())
    } else {
        Ok(Json(usr.unwrap()))
    }
}
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

#[options("/user/new")]
pub fn new_user_preflight() -> NoContent {
    NoContent
}

#[post("/user/new", data = "<user>")]
pub fn new_user(user: Json<WithSecret<NewUserJson>>) -> Json<User> {
    assert!(
        user.secret == crate::SECRET,
        "Wrong secret: {}, {}",
        user.secret,
        crate::SECRET
    );

    let newuser = NewUser {
        name: user.data.name.clone(),
        email: user.data.email.clone(),
    };
    let usr = diesel::insert_into(users::table)
        .values(&newuser)
        .returning(User::as_returning())
        .get_result(&mut establish_connection());
    Json(usr.unwrap())
}

#[options("/user/update")]
pub fn update_user_preflight() -> NoContent {
    NoContent
}

#[post("/user/update", data = "<data>")]
pub fn update_user(data: Json<WithSecret<UserJson>>) -> Result<Json<User>, ()> {
    if data.secret != crate::SECRET {
        println!("Wrong secret: {}, {}", data.secret, crate::SECRET);
        return Err(());
    }

    println!("Updating user {:?}", data);

    let rslt = diesel::update(users::table.filter(users::id.eq(data.data.id)))
        .set((
            users::name.eq(data.data.name.clone()),
            users::email.eq(data.data.email.clone()),
        ))
        .get_result(&mut establish_connection());

    match rslt {
        Ok(usr) => {
            println!("Updated user {:?}", usr);
            Ok(Json(usr))
        }
        Err(_) => Err(()),
    }
}

#[post("/user/get", data = "<data>")]
pub fn get_user(data: Json<WithSecret<UserSelectJson>>) -> Result<Json<User>, ()> {
    if data.secret != crate::SECRET {
        println!("Wrong secret: {}, {}", data.secret, crate::SECRET);
        return Err(());
    }
    let mut fltr = users::table.into_boxed();
    if let Some(id) = data.data.id {
        fltr = fltr.filter(users::id.eq(id));
    }
    if let Some(name) = &data.data.name {
        fltr = fltr.filter(users::name.eq(name));
    }
    if let Some(email) = &data.data.email {
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

#[options("/user/list/all")]
pub fn list_users_preflight() -> NoContent {
    NoContent
}

#[post("/user/remove", data = "<data>")]
pub fn remove_user(data: Json<WithSecret<UserSelectJson>>) -> Result<Json<User>, ()> {
    println!("{:?}", data);

    let conn = &mut establish_connection();

    if data.secret != crate::SECRET {
        println!("Wrong secret: {}, {}", data.secret, crate::SECRET);
        return Err(());
    }

    let mut fltr = users::table.into_boxed();

    if let Some(id) = data.data.id {
        fltr = fltr.filter(users::id.eq(id));
    }
    if let Some(name) = &data.data.name {
        fltr = fltr.filter(users::name.eq(name));
    }
    if let Some(email) = &data.data.email {
        fltr = fltr.filter(users::email.eq(email));
    }

    let user: User = fltr.first(conn).expect("Error loading users");

    match delete_user(user.id, conn) {
        Ok(usr) => Ok(usr),
        Err(e) => {
            println!("Error: {}", e);
            Err(())
        }
    }
}

#[options("/user/remove")]
pub fn remove_user_preflight() -> NoContent {
    NoContent
}
