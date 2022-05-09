use super::shared::opt_iso8601_datetime_from_string;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;

/// This struct represents a data range
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct DateRange {
    #[serde(with = "opt_iso8601_datetime_from_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<OffsetDateTime>,
    #[serde(with = "opt_iso8601_datetime_from_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<OffsetDateTime>,
}
