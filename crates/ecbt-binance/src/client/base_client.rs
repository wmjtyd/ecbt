use super::Transport;

/// The ecbt-binance client
#[derive(Clone)]
pub struct BaseClient {
    pub transport: Transport,
}
