use super::AccountUpdate;
use super::AggregateTrade;
use super::CandlestickMessage;
use super::Depth;
use super::MiniTicker;
use super::OrderBook;
use super::Ticker;
use super::TradeMessage;
use super::UserOrderUpdate;
use serde::Serialize;

/// This enum represents the types of websocket messages
#[derive(Debug, Clone, Serialize)]
pub enum BinanceWebsocketMessage {
    UserOrderUpdate(UserOrderUpdate),
    UserAccountUpdate(AccountUpdate),
    AggregateTrade(AggregateTrade),
    Trade(TradeMessage),
    Candlestick(CandlestickMessage),
    MiniTicker(MiniTicker),
    MiniTickerAll(Vec<MiniTicker>),
    Ticker(Ticker),
    TickerAll(Vec<Ticker>),
    OrderBook(OrderBook),
    Depth(Depth),
    Ping,
    Pong,
    Close,
    Binary(Vec<u8>), // Unexpected, unparsed
}
