use crate::model::market_pair::MarketPair;
use crate::model::TimeInForce;
use derive_more::Constructor;
use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use serde::Serialize;

/// This struct represents an open limit order
#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct EcbtOrderRequest {
    pub client_order_id: Option<String>,
    pub market_pair: MarketPair,
    pub size: Decimal,
    pub price: Decimal,
    pub time_in_force: TimeInForce,
    pub post_only: bool,
}
