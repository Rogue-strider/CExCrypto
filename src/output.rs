use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderResponse{
    pub order_id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOrderResponse{
    pub filled_qty: u32,
    pub average_price: u32
}