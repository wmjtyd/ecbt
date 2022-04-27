use std::borrow::Borrow;

use ecbt::{
    prelude::{
        market_pair::{Currency, MarketPair},
        GetPriceTickerRequest,
    },
    Ecbt,
};

use ecbt_binance::{Binance, BinanceParameters};
use ecbt_exchange::ExchangeMarketData;

#[tokio::main]
async fn main() {
    let ecbt = Ecbt::http::<Binance>(BinanceParameters::sandbox())
        .await
        .unwrap();
    let request = GetPriceTickerRequest {
        market_pair: MarketPair(Currency::ETH, Currency::USDT),
    };
    let s = ecbt.get_price_ticker(request.borrow()).await.unwrap();
    println!("{:?}", s);
}
