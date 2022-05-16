use dotenv::dotenv;
use ecbt_exchange::model::currency::Currency;
use ecbt_exchange::model::market_pair::MarketPair;
use ecbt_ftx::{
    ftx_options::Options,
    rest::{GetMarket, Rest, Result},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let options = Options::from_env_us();
    let option_clone = options.clone();
    let api = Rest::new(options);
    let market = option_clone
        .clone()
        .to_market(MarketPair(Currency::ETH, Currency::USDT));
    let price = api.request(GetMarket::new(&market.to_owned())).await?.price;
    println!("1 ETH is worth {} USDT.", price.unwrap());//BTC/USD

    Ok(())
}
