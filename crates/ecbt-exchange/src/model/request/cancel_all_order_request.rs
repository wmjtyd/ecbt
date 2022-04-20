use crate::model::market_pair::MarketPair;

use serde::Deserialize;
use serde::Serialize;

/// This struct represents the cancellation of all orders
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CancelAllOrdersRequest {
    pub market_pair: Option<MarketPair>,
}
