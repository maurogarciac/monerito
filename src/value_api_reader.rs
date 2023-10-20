use chrono::{Local, DateTime};

const CRYPTO_URL: &str = "https://min-api.cryptocompare.com/data/price?fsym=XMR&tsyms=BTC,USD";
const PESOS_URL: &str = "https://dolarapi.com/v1/dolares.blue";

#[tokio::main]
pub fn read_crypto() {
    let response = reqwest::get(CRYPTO_URL)
        .await.unwrap().json::<Crypto>()
        .await;
    match response = {
        Ok(crypto) => {
            info!("Processing USD and BTC values");
            for result in crypto.result {
                process_order(result);
            }
        }
        Err(e) => {
            error!("Crypto API response cannot be parsed! {}", e)
        }
    };
}

#[tokio::main]
pub fn read_pesos(){
    let response = reqwest::get(PESOS_URL)
        .await.unwrap().json::<Pesos>()
        .await;
    match response = {
        Ok(pesos) => {
            info!("Processing ARS value");
            for result in pesos.result {
                process_order(result);
            }
        }
        Err(e) => {
            error!("Pesos API response cannot be parsed! {}", e)
        }
    };
}   


async fn process_values(result: Values) {
    let time = Local::now();
    let last_hour = Local::now() - Duration::minutes(60);
    let last_minute = Local::now() - Duration::minutes(1);
    let after_hour = time > last_hour;
    let after_minute = time > last_minute;

    if after_hour {
        let peso = read_pesos();
    }
    if after_minute {
        let crypto = read_crypto();
    }

    peso * crypto.get("USD")
}
