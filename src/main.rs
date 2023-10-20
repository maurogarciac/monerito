#[macro_use] extern crate rocket;

use value_api_reader::read_orders;

mod value_api_reader;
mod model;

#[get("/")]
fn index() -> &'static str {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));
    read_values()
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
