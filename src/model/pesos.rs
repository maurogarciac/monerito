use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Pesos {
    pub compra: i64,
    pub venta: i64,
    pub fecha_actualizacion: String,
}
