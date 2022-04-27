use super::shared::Result;
use super::ExchangeInfo;
use super::MarketPairHandle;

pub fn get_pair(name: &str, exchange_info: &ExchangeInfo) -> Result<MarketPairHandle> {
    exchange_info.get_pair(name)
}
