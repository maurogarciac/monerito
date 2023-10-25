#[macro_use] extern crate rocket;

use std::io::Read;
use std::task::Poll::{Pending, Ready};
use rocket::tokio::task;
use rocket::tokio::task::JoinHandle;
use value_api_reader::process_values;

mod value_api_reader;
mod model;

#[get("/")]
fn index() -> &'static str {
    "yeah this doesn't really work the way I expected it to work"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
