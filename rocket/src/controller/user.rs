use crate::model::user::*;
use rocket::{serde::json::Json, State};
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[get("/<id>")]
pub async fn info(id: String, conn: &State<SqlitePool>) -> Value {
    println!("{}", id);
    let user_data = user_info(conn, id).await;

    json!({
        "id": user_data.id.to_string(),
        "nickname": user_data.nickname.to_string(),
        "create_date": user_data.create_date.to_string(),
        "update_date": user_data.update_date.to_string()
    })
}

#[post("/<id>", data = "<user>")]
pub async fn create(id: String, user: Json<User>, conn: &State<SqlitePool>) -> () /*Result<Json<User>, Json<ErrorResponse>>*/
{
    let user = User {
        id,
        pw: user.pw.to_string(),
        nickname: user.nickname.to_string(),
        ..Default::default()
    };

    println!("{user:#?}");

    let _tmp = create_user(conn, user).await;
}

#[delete("/<id>", data = "<user>")]
pub async fn delete(id: String, user: Json<User>, conn: &State<SqlitePool>) -> &'static str {
    println!("{:#?}", user);

    let user = User {
        id,
        pw: user.pw.to_string(),
        ..Default::default()
    };

    delete_user(conn, user).await;
    ""
}
