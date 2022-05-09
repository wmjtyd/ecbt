use super::shared::iso8601_datetime_from_string;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;

/// This enum represents a response of a time in force
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "time_in_force")]
pub enum OrderTimeInForceResponse {
    GTC,
    GTT {
        #[serde(with = "iso8601_datetime_from_string")]
        expire_time: OffsetDateTime,
    },
    IOC,
    FOK,
}
