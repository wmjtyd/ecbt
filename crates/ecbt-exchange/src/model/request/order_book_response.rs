use crate::model::AskBid;

use serde::Deserialize;
use serde::Serialize;

/// This struct represents an order book response
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OrderBookResponse {
    pub update_id: Option<u64>,
    pub last_update_id: Option<u64>,
    pub bids: Vec<AskBid>,
    pub asks: Vec<AskBid>,
}
