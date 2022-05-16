use dotenv::dotenv;
use ecbt_exchange::model::currency::Currency;
use ecbt_exchange::model::market_pair::MarketPair;
use ecbt_ftx::ftx_options::{Endpoint, Options};
use ecbt_ftx::ws::Result;
use ecbt_ftx::ws::{Channel, Data, Orderbook, Ws};
use futures::stream::StreamExt;
use std::io;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let options = Options {
        endpoint: Endpoint::Us,
        key: Some("obAqJG1_4_qX4deJ-UwVm6drZ89mafGhd9YkNm8l".to_string()),
        secret: Some("7dqybPY84KtZMxL_YAoKMQlQDNuFPAbbF73a8tI5".to_string()),
        subaccount: Some("saber".to_string()),
    };
    let option_clone = options.clone();
    let mut websocket = Ws::connect(options).await?;
    // let mut websocket = Ws::connect(Options::from_env_us()).await?;
    // let market = String::from("BTC-PERP");
    let market = option_clone
        .clone()
        .to_market(MarketPair(Currency::ETH, Currency::USDT));
    let mut orderbook = Orderbook::new(market.to_owned());

    websocket
        .subscribe(vec![
            Channel::Trades(market.to_owned()),
            Channel::Orderbook(market.to_owned()),
        ])
        .await?;

    loop {
        let data = websocket.next().await.expect("No data received")?;

        match data {
            (_, Data::Trade(trade)) => {
                println!(
                    "\n{:?} {} {} at {} - liquidation = {}",
                    trade.side, trade.size, market, trade.price, trade.liquidation
                );
            }
            (_, Data::OrderbookData(orderbook_data)) => {
                orderbook.update(&orderbook_data);
                // print!("."); // To signify orderbook update
                println!("{:?}", orderbook);
                io::stdout().flush().unwrap(); // Emits the output immediately
            }
            _ => panic!("Unexpected data type"),
        }
    }
}
