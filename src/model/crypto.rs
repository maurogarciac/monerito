use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Crypto {
    pub btc: f64,
    pub usd: f64,
}
