#[cfg(test)]
mod tests;

#[macro_use]
extern crate rocket;

use sqlx::sqlite::SqlitePool;

mod controller;
mod database;
mod model;

#[get("/")]
async fn index(/* conn: State<DbConn> */) -> &'static str {
    "Hello, world"
}

// 0.5
#[rocket::main]
async fn main() {
    let conn = SqlitePool::connect("sqlite:../database.sqlite")
        .await
        .unwrap();

    database::init_db(&conn).await;

    rocket::build()
        .manage(conn)
        .mount("/", routes![index])
        .mount("/user", controller::routes())
        .launch()
        .await
        .unwrap();
}
