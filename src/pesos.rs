use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Pesos {
    pub valores: Vec<ValorPesos>,
}

#[derive(Deserialize, Debug)]
pub struct ValorPesos {
    pub compra: i64,
    pub venta: i64,
    pub fechaActualizacion: String,
}
