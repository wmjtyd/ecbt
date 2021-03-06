use crate::model::market_pair::MarketPair;
use crate::model::{OrderStatus, Paginator};

use serde::Deserialize;
use serde::Serialize;

/// This struct represents the historic of the orders
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrderHistoryRequest {
    pub market_pair: Option<MarketPair>,
    pub order_status: Option<Vec<OrderStatus>>,
    pub paginator: Option<Paginator>,
}
