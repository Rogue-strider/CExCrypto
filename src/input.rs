use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderInput {
    pub price: f64,
    pub quantity: f64,
    pub user_id: String,
    pub side: Side,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOrder {
    pub order_id: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Depth{
    pub bids: Vec<[u32; 2]>,  //means vector of array which has 2 elemenet
    pub asks: Vec<[u32; 2]>,
    pub last_updateid: String
}