#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rust web!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
