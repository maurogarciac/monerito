#[macro_use] extern crate rocket;

use std::io::Read;
use value_api_reader::process_values;

mod value_api_reader;
mod model;

#[get("/")]
async fn index() -> String {
    process_values().await.to_string()
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
