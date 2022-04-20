# ecbt
![CI status](https://github.com/oss-jtyd/ecbt/actions/workflows/build.yml/badge.svg)
[![License](https://img.shields.io/badge/License-BSD_2--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![GitHub release](https://img.shields.io/github/v/release/oss-jtyd/ecbt.svg)](https://github.com/oss-jtyd/ecbt/releases)

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
tokio = { version = "1", features = ["full"] }
ecbt = { git = "https://github.com/oss-jtyd/ecbt", tag = "1.0.0" }
```

### HTTP

```rust
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
```

### WebSocket

```rust
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