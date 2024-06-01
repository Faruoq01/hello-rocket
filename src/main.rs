#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/name")]
fn world() -> &'static str {
    "Hello, Faruk!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/hello", routes![world])
    .mount("/hi", routes![world])
}