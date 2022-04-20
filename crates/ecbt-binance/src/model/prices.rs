use super::SymbolPrice;
use serde::Deserialize;
use serde::Serialize;

/// This enum represents the prices
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Prices {
    AllPrices(Vec<SymbolPrice>),
}
