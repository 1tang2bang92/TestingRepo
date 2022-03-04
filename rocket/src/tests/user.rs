use rocket::local::Client;
use rocket::http::ContentType;

#[test]
fn get_user() {
    let client = Client::new(rocket::ignite()).expect("valid rocket");

    let req = client.get("/");
}