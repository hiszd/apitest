// use rocket::futures::FutureExt;
use diesel::prelude::*;
#[allow(unused_imports)]
use rocket::futures::{SinkExt, StreamExt};
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

use crate::establish_connection;
use crate::model::*;
use crate::schema::*;
use crate::types::json::shared::JustSecret;
use crate::types::json::shared::WithSecret;
use crate::types::json::user::*;
use crate::Topic;
use crate::Update;
use crate::STATE;

async fn delete_user(
  user_id: i32,
  conn: &mut PgConnection,
) -> Result<Json<User>, diesel::result::Error> {
  println!("Deleting user {}", user_id);
  let usr = users::table
    .filter(users::id.eq(user_id))
    .select(User::as_select())
    .get_result(conn);
  let tiks = tickets_authors::table
    .inner_join(tickets::table)
    .filter(tickets_authors::author_id.eq(user_id))
    .select(Ticket::as_select())
    .load(&mut establish_connection())
    .unwrap();
  tiks.iter().for_each(|tik| {
    diesel::delete(tickets_authors::table)
      .filter(tickets_authors::ticket_id.eq(tik.id))
      .execute(conn)
      .unwrap();
    diesel::delete(tickets::table)
      .filter(tickets::id.eq(tik.id))
      .execute(conn)
      .unwrap();
  });
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
    STATE.lock().await.trigger_update(Update {
      topic: Topic::Users,
      ids: vec![user_id.to_string()],
    });
    println!("Reset users");
    Ok(Json(usr.unwrap()))
  }
}

#[options("/user/new")]
pub fn new_user_preflight() -> NoContent {
  NoContent
}

#[post("/user/new", data = "<user>")]
pub async fn new_user(user: Json<WithSecret<NewUserJson>>) -> Json<User> {
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

  let usrunwrap = usr.unwrap();

  STATE.lock().await.trigger_update(Update {
    topic: Topic::Users,
    ids: vec![usrunwrap.id.to_string()],
  });
  println!("Reset users");

  Json(usrunwrap)
}

#[options("/user/update")]
pub fn update_user_preflight() -> NoContent {
  NoContent
}

#[post("/user/update", data = "<data>")]
pub async fn update_user(data: Json<WithSecret<UserJson>>) -> Result<Json<User>, ()> {
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

  STATE.lock().await.trigger_update(Update {
    topic: Topic::Users,
    ids: vec![data.data.id.to_string()],
  });
  println!("Reset users");

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
    fltr
      .first(&mut establish_connection())
      .expect("Error loading users"),
  ))
}

#[options("/user/get")]
pub fn get_user_preflight() -> NoContent {
  NoContent
}

#[post("/user/list/all", data = "<data>")]
pub fn list_users(data: Json<JustSecret>) -> Result<Json<Vec<User>>, ()> {
  if data.secret != crate::SECRET {
    println!("Wrong secret: {}, {}", data.secret, crate::SECRET);
    return Err(());
  }
  Ok(Json(
    users::table
      .select(User::as_select())
      .load(&mut establish_connection())
      .unwrap(),
  ))
}

#[options("/user/list/all")]
pub fn list_users_preflight() -> NoContent {
  NoContent
}

#[post("/user/remove", data = "<data>")]
pub async fn remove_user(data: Json<WithSecret<UserSelectJson>>) -> Result<Json<User>, ()> {
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

  match delete_user(user.id, conn).await {
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
