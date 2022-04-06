mod user;
use crate::user::*;
use serde_json::{from_str, Value};

mod security;

use rocket::State;
use sqlx::sqlite::SqlitePool;

#[macro_use]
extern crate rocket;

#[get("/user/<id>")]
async fn get_user(id: String, conn: &State<SqlitePool>) -> String {
    User::get_user_by_id(id, conn).await;
    "test".to_string()
}

#[post("/user/<id>", data = "<user>")]
async fn create_user(id: String, user: String, conn: &State<SqlitePool>) -> String {
    let body: Value = from_str(&user).unwrap();

    //TODO: plz append security trait
    let pw = body.get("pw").unwrap().to_string();
    let nickname = body.get("nickname").unwrap();

    let _user = UserBuilder::default()
        .create(id.to_string(), pw.to_string(), nickname.to_string())
        .build()
        .create_user(conn)
        .await;

    format!("{id} usr succesfully created")
}

#[delete("/user/<id>", data = "<user>")]
async fn delete_user(id: String, user: String, conn: &State<SqlitePool>) -> String {
    "test".to_string()
}
#[patch("/user/<id>", data = "<user>")]
async fn change_nickname(id: String, user: String, conn: &State<SqlitePool>) -> String {
    "test".to_string()
}

fn routes() -> Vec<rocket::Route> {
    routes![create_user, get_user, delete_user, change_nickname]
}

#[launch]
async fn rocket() -> _ {
    let conn = SqlitePool::connect("sqlite:../database.sqlite")
        .await
        .unwrap();

    rocket::build().manage(conn).mount("/api", routes())
}
