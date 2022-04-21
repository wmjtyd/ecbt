use std::borrow::Borrow;

use ecbt_binance::{Binance, BinanceCredentials, BinanceParameters};
use ecbt_exchange::exchange::Environment::Sandbox;
use ecbt_exchange::model::currency::Currency;
use ecbt_exchange::model::market_pair::MarketPair;
use ecbt_exchange::model::GetPriceTickerRequest;
use ecbt_exchange::shared::Result;
use ecbt_exchange::{Exchange, ExchangeMarketData};

#[tokio::main]
async fn main() -> Result<()> {
    let credentials = BinanceCredentials {
        api_key: "xxx".to_string(),
        api_secret: "xxx".to_string(),
    };
    let param = BinanceParameters {
        environment: Sandbox,
        credentials: Some(credentials),
    };
    let binance = Binance::new(param).await?;
    let request = GetPriceTickerRequest {
        market_pair: MarketPair(Currency::ETH, Currency::USDT),
    };
    let s = binance.get_price_ticker(request.borrow()).await?;
    println!("{:?}", s);
    Ok(())
}
