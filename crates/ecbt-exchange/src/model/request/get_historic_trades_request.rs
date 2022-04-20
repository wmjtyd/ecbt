use crate::model::Paginator;

use serde::Deserialize;
use serde::Serialize;

/// This struct represents the historic of the trades
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetHistoricTradesRequest {
    pub market_pair: String,
    pub paginator: Option<Paginator>,
}
