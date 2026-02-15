use std::collections::HashMap;
use std::collections::BTreeMap;
use crate::{inputs::Side, outputs::Depth};

pub struct Orderbook {
    pub bids: BTreeMap<u32, Vec<UserOrder>>, // highest price priority
    pub asks: BTreeMap<u32, Vec<UserOrder>>, // lowest price priority
    pub order_id_index: u32
}

pub struct UserOrder {
    pub user_id: u32,
    pub qty: u32,
    pub order_id: u32
}

impl Orderbook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            order_id_index: 0
        }
    }
}

impl Orderbook {
    pub fn create_order(&mut self, price: u32, quantity: u32, user_id: u32, side: Side) {
        let order_id = self.order_id_index;
        self.order_id_index = self.order_id_index + 1 ;
        if side == Side::Buy {
            self.bids.entry(price).or_insert(Vec::new()).push(UserOrder {
                user_id,
                qty: quantity,
                order_id: order_id
            });
        } else {
            self.asks.entry(price).or_insert(Vec::new()).push(UserOrder {
                user_id,
                qty: quantity,
                order_id: order_id
            });
        }
        
    }

    pub fn get_depth(&self) -> Depth {
        let mut bids_vec = Vec::new();
        let mut asks_vec = Vec::new();

        // highest bid should be atfirst in exchnages 
        for (price, orders) in self.bids.iter().rev() {
            let total_qty: u32 = orders.iter().map(|o| o.qty).sum();
            bids_vec.push([*price, total_qty]);
        }

        // lowest ask first (default order)
        for (price, orders) in &self.asks {
            let total_qty: u32 = orders.iter().map(|o| o.qty).sum();
            asks_vec.push([*price, total_qty]);
        }

        Depth {
            bids: bids_vec,
            asks: asks_vec,
            lastUpdateId: self.order_id_index.to_string(),
        }
    }
}

