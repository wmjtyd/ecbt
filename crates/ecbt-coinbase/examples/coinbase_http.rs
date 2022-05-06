use ecbt_coinbase::{Coinbase, CoinbaseParameters};
use ecbt_exchange::{
    model::{
        market_pair::{Currency, MarketPair},
        OrderBookRequest,
    },
    Exchange, ExchangeMarketData,
};

#[tokio::main]
async fn main() {
    let coinbase = Coinbase::new(CoinbaseParameters::sandbox())
        .await
        .expect("Couldn't create coinbase client");
    let market_pair = MarketPair(Currency::BTC, Currency::ETH);
    let order_book = coinbase
        .order_book(&OrderBookRequest { market_pair })
        .await
        .expect("Couldn't get order book");
    println!("{:?}", order_book);
}
