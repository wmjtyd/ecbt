use ecbt::{
    model::websocket::{EcbtWebSocketMessage, Subscription, WebSocketResponse},
    prelude::{
        market_pair::{Currency, MarketPair},
        ExchangeStream,
    },
    Ecbt,
};

use ecbt_binance::{BinanceParameters, BinanceWebsocket};

#[tokio::main]
async fn main() {
    let ecbt = Ecbt::ws::<BinanceWebsocket>(BinanceParameters::sandbox())
        .await
        .unwrap();
    let market = MarketPair(Currency::ETH, Currency::BTC);

    ecbt.subscribe(Subscription::OrderBookUpdates(market), move |m| {
        let r = m.as_ref();

        if let Ok(WebSocketResponse::Generic(EcbtWebSocketMessage::OrderBook(order_book))) = r {
            println!("{:?}", order_book)
        } else if let Err(err) = r {
            println!("{:#?}", err);
        }
    })
    .await
    .expect("Failed to subscribe to orderbook on Binance");

    std::thread::sleep(std::time::Duration::from_millis(5000));
}
