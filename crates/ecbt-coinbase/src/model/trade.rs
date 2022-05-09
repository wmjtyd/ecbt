use super::shared::iso8601_datetime_from_string;
use super::shared::string_to_decimal;
use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;

/// This struct represents a trade
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub trade_id: u64,
    #[serde(with = "iso8601_datetime_from_string")]
    pub time: OffsetDateTime,
    pub size: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    pub side: String,
}
