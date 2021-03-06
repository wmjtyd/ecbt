use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use serde::Serialize;

/// This struct represents the ask and bid
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default, PartialEq)]
pub struct AskBid {
    pub price: Decimal,
    pub qty: Decimal,
}
