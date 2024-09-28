use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CryptoPrice {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
}

#[derive(Debug)]
pub struct Portfolio {
    pub cryptocurrency: String,
    pub amount: f64,
}
