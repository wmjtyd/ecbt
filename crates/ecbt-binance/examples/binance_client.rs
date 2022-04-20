use std::borrow::Borrow;

use ecbt_binance::{Binance, BinanceCredentials, BinanceParameters};
use ecbt_exchange::exchange::Environment::Sandbox;
use ecbt_exchange::model::currency::Currency;
use ecbt_exchange::model::market_pair::MarketPair;
use ecbt_exchange::model::{GetPriceTickerRequest, OrderBookRequest};
use ecbt_exchange::shared::Result;
use ecbt_exchange::{Exchange, ExchangeMarketData};

#[tokio::main]
async fn main() -> Result<()> {
    let credentials = BinanceCredentials {
        api_key: "uGiJlKQi4NZxwr1wnlbdAX5K4RLwj4ew4MDfxokjEFJSGxt61mCtQvlYC9IV83bM".to_string(),
        api_secret: "KWuEHMSLP9DoU2akirSYm3NqgWwamLPBsZpYoBOiRMQqgiOv639pEsJFhu04K4Ux".to_string(),
    };
    let param = BinanceParameters {
        environment: Sandbox,
        credentials: Some(credentials),
    };
    let binance = Binance::new(param).await?;
    let request = GetPriceTickerRequest {
        market_pair: MarketPair {
            0: Currency::BTC,
            1: Currency::USD,
        },
    };
    let s = binance.get_price_ticker(request.borrow()).await?;
    println!("{:?}", s);
    Ok(())
}
