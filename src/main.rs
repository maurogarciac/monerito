#[macro_use] extern crate rocket;

mod value_api_reader;
mod model;

#[get("/")]
fn index() -> String {
    value_api_reader::monero_price()
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
