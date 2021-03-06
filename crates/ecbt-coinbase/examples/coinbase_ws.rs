use std::time::Duration;

use ecbt_coinbase::model::websocket::{CoinbaseSubscription, CoinbaseWebsocketMessage};
use ecbt_coinbase::{CoinbaseParameters, CoinbaseWebsocket};
use ecbt_exchange::model::websocket::WebSocketResponse;
use ecbt_exchange::shared::Result;
use ecbt_exchange::stream::ExchangeStream;

#[tokio::main]
async fn main() -> Result<()> {
    let binance_ws = CoinbaseWebsocket::new(CoinbaseParameters::sandbox()).await?;
    let symbol = "ETH-BTC".to_string();
    binance_ws
        .subscribe(CoinbaseSubscription::Matches(symbol), call_back)
        .await?;

    std::thread::sleep(Duration::from_secs(100));
    Ok(())
}

fn call_back(res: &Result<WebSocketResponse<CoinbaseWebsocketMessage>>) {
    if let Ok(WebSocketResponse::Raw(CoinbaseWebsocketMessage::Ticker(msg))) = res {
        println!("{:?}", msg);
    }
}
