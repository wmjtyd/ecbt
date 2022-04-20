use super::Paginator;
use serde::Deserialize;
use serde::Serialize;

/// This struct represents a trade history request
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistoryReq {
    pub symbol: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paginator: Option<Paginator>,
}
