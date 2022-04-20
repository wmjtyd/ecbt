use std::time::Duration;

use ecbt_binance::model::websocket::{BinanceSubscription, BinanceWebsocketMessage};
use ecbt_binance::{BinanceCredentials, BinanceParameters, BinanceWebsocket};
use ecbt_exchange::exchange::Environment::Sandbox;
use ecbt_exchange::model::currency::Currency;
use ecbt_exchange::model::market_pair::MarketPair;
use ecbt_exchange::model::websocket::WebSocketResponse;
use ecbt_exchange::shared::Result;
use ecbt_exchange::stream::ExchangeStream;

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
    let binance_ws = BinanceWebsocket::new(param).await?;
    let symbol = MarketPair(Currency::ETH, Currency::USDT).to_string();
    binance_ws
        .subscribe(BinanceSubscription::Ticker(symbol), call_back)
        .await?;

    std::thread::sleep(Duration::from_secs(10));
    Ok(())
}

fn call_back(res: &Result<WebSocketResponse<BinanceWebsocketMessage>>) {
    if let Ok(WebSocketResponse::Raw(BinanceWebsocketMessage::Ticker(msg))) = res {
        println!("{:?}", msg);
    }
}
