use rocket::tokio;
use chrono::{Local, DateTime, Duration};
use crate::model::crypto::Crypto;
use crate::model::pesos::Pesos;
use futures::executor;

const CRYPTO_URL: &str = "https://min-api.cryptocompare.com/data/price?fsym=XMR&tsyms=BTC ,USD";
const PESOS_URL: &str = "https://dolarapi.com/v1/dolares.blue";

async fn read_crypto() -> Crypto {
    let response: Crypto = reqwest::get(CRYPTO_URL)
        .await.unwrap().json::<Crypto>().await.unwrap();
    response
}

async fn read_pesos() -> Pesos {
    let response = reqwest::get(PESOS_URL)
        .await.unwrap().json::<Pesos>()
        .await;
    response.unwrap()
}   


async fn process_values() -> String {
    let time = Local::now();
    let last_hour = Local::now() - Duration::minutes(60);
    let last_minute = Local::now() - Duration::minutes(1);
    let after_hour = time > last_hour;
    let after_minute = time > last_minute;


    let mut peso: Option<Pesos> = None;
    let mut crypto: Option<Crypto> = None;
    if after_hour {
        peso = Some(read_pesos().await);
    }
    if after_minute {
        crypto = Some(read_crypto().await);
    }
    let mut value;

    return if peso.is_some() && crypto.is_some() {
        value = peso.unwrap().compra as f64 * crypto.unwrap().usd;
        println!("{}", value);
        value.to_string()
    } else {
        "yeah this doesn't work".to_string()
    }
}

pub fn monero_price() -> String {
    let v: String = executor::block_on(process_values());
    v
}
