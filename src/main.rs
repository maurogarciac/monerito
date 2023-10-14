#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "monerito.ar"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
