use super::Ticker;
use serde::Deserialize;
use serde::Serialize;

/// This enum represents a book ticker
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum BookTickers {
    AllBookTickers(Vec<Ticker>),
}
