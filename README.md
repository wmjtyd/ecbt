# ecbt(Exchange Connect Base Tool)
[![CI status](https://github.com/wmjtyd/ecbt/actions/workflows/build.yml/badge.svg)](https://github.com/wmjtyd/ecbt/actions?query=workflow%3ABuild+branch%3Amain)
[![CI status](https://github.com/wmjtyd/ecbt/actions/workflows/security.yml/badge.svg)](https://github.com/wmjtyd/ecbt/actions?query=workflow%3ASecurity)
[![License](https://img.shields.io/badge/License-BSD_2--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![Crates.io](https://img.shields.io/crates/v/ecbt.svg)](https://crates.io/crates/ecbt)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.54+-lightgray.svg)](https://github.com/wmjtyd/ecbt)
[![Join at Slack](https://img.shields.io/badge/chat-slack-brightgreen)](https://join.slack.com/t/wmjtyd/shared_invite/zt-17sfuaoj7-~0WmpuFL_NTqS~4WTNEXTg)

High performance cryptocurrency trading API with support for user defined networking.

* Based on Rust, memory safe by default.
* Support for websockets and user defined networking.
* Easy to add support for additional exchanges.

## License

[BSD 2-Clause License](https://opensource.org/licenses/BSD-2-Clause)


## Usage

Add dependencies in `Cargo.toml`:

```toml
[dependencies]
ecbt = "0.1"
ecbt-exchange = "0.1"
ecbt-binance = "0.1"
tokio = { version = "1", features = ["full"] }
```

### HTTP

```rust
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
```

### WebSocket

```rust
use ecbt::{
    prelude::{
        market_pair::{Currency, MarketPair},
        websocket::{EcbtWebSocketMessage, Subscription, WebSocketResponse},
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
```

## Contributions

Because there are many exchanges, if ecbt has not been implemented, you may need to implement it yourself.

## Contact Us

Contact or join us [@oss-jtyd](https://github.com/oss-jtyd).

### WIP

- [done] Project Skeleton
- [done] Binance support
- [todo] Nash support
- [todo] Huobi support
- [todo] Coinbase support
- [todo] Apifiny support
- [todo] Okex support