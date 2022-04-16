use crate::model::market_pair::MarketPair;
use crate::model::Paginator;
use serde::Deserialize;
use serde::Serialize;

/// This struct represents the trade history
#[derive(Serialize, Deserialize, Default)]
pub struct TradeHistoryRequest {
    pub market_pair: Option<MarketPair>,
    pub order_id: Option<String>,
    pub paginator: Option<Paginator>,
}
