#[macro_use] extern crate rocket;

mod value_api_reader;
mod model;

#[get("/")]
fn index() -> &'static str {
    let mut p: String = value_api_reader::monero_price().to_owned();
    return p.as_str();
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
