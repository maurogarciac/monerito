const CRYPTO_URL: &str = "https://min-api.cryptocompare.com/data/price?fsym=XMR&tsyms=BTC,USD";
const PESOS_URL: &str = "https://dolarapi.com/v1/dolares.blue";

#[tokio::]
pub fn read_orders{} {
    let response = request::get(ORDERS_URL)
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

async fn process_order(result: )
