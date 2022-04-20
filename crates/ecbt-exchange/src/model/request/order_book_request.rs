use crate::model::market_pair::MarketPair;

use serde::Deserialize;
use serde::Serialize;

/// This struct represents an order book request
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OrderBookRequest {
    pub market_pair: MarketPair,
}
