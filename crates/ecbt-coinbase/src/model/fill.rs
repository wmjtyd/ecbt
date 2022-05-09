use super::shared::{iso8601_datetime_from_string, string_to_decimal};
use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;

/// This struct represents a fill order
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fill {
    pub trade_id: u64,
    pub product_id: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
    pub order_id: String,
    #[serde(with = "iso8601_datetime_from_string")]
    pub created_at: OffsetDateTime,
    pub liquidity: String,
    #[serde(with = "string_to_decimal")]
    pub fee: Decimal,
    pub settled: bool,
    pub side: String,
}
