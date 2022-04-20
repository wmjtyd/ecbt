use crate::model::market_pair::MarketPair;
use crate::model::{Interval, Paginator};

use serde::Deserialize;
use serde::Serialize;

/// This struct represents the historic of the rates
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetHistoricRatesRequest {
    pub market_pair: MarketPair,
    pub paginator: Option<Paginator>,
    pub interval: Interval,
}
