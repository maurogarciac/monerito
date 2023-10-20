use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Crypto {
    pub values: Vec<Values>,
}

#[derive(Deserialize, Debug)]
pub struct Values {
    pub btc: f64,
    pub usd: f64,
}
