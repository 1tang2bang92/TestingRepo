use rocket::Route;

mod user;

pub fn routes() -> Vec<Route> {
    routes![user::info, user::create, user::delete]
}
