//! Ecbt integrates a high-performance cryptocurrency transaction API, which can be easily used by users.
//!
//! # Example
//!
//! The Binance Http Api of Ecbt is:
//!
//! ```rust,no_run
//! use std::borrow::Borrow;
//!
//! use ecbt::{
//!     prelude::{
//!         market_pair::{Currency, MarketPair},
//!         GetPriceTickerRequest,
//!     },
//!     Ecbt,
//! };
//!
//! use ecbt_binance::{Binance, BinanceParameters};
//! use ecbt_exchange::ExchangeMarketData;
//!
//! #[tokio::main]
//! async fn main() {
//!     let ecbt = Ecbt::http::<Binance>(BinanceParameters::sandbox())
//!         .await
//!         .unwrap();
//!     let request = GetPriceTickerRequest {
//!         market_pair: MarketPair(Currency::ETH, Currency::USDT),
//!     };
//!     let s = ecbt.get_price_ticker(request.borrow()).await.unwrap();
//!     println!("{:?}", s);
//! }
//! ```
//!
//! The Binance WebSocket of Ecbt is:
//!
//! ```rust,no_run
//! use ecbt::{
//!     model::websocket::{EcbtWebSocketMessage, Subscription, WebSocketResponse},
//!     prelude::{
//!         market_pair::{Currency, MarketPair},
//!         ExchangeStream,
//!     },
//!     Ecbt,
//! };
//!
//! use ecbt_binance::{BinanceParameters, BinanceWebsocket};
//!
//! #[tokio::main]
//! async fn main() {
//!     let ecbt = Ecbt::ws::<BinanceWebsocket>(BinanceParameters::sandbox())
//!         .await
//!         .unwrap();
//!     let market = MarketPair(Currency::ETH, Currency::BTC);
//!
//!     ecbt.subscribe(Subscription::OrderBookUpdates(market), move |m| {
//!         let r = m.as_ref();
//!
//!         if let Ok(WebSocketResponse::Generic(EcbtWebSocketMessage::OrderBook(order_book))) = r {
//!             println!("{:?}", order_book)
//!         } else if let Err(err) = r {
//!             println!("{:#?}", err);
//!         }
//!     })
//!     .await
//!     .expect("Failed to subscribe to orderbook on Binance");
//!
//!     std::thread::sleep(std::time::Duration::from_millis(5000));
//! }
//!
//! ```

pub mod exchange;
pub mod prelude;

use ecbt_exchange::stream::ExchangeStream;
use ecbt_exchange::Exchange;

pub use crate::exchange::errors;
pub use crate::exchange::model;
use crate::exchange::shared::Result;

pub struct Ecbt {}

impl Ecbt {
    pub async fn http<E: Exchange>(parameters: E::InitParams) -> Result<E> {
        E::new(parameters).await
    }

    pub async fn ws<E: ExchangeStream>(parameters: E::InitParams) -> Result<E> {
        E::new(parameters).await
    }
}
