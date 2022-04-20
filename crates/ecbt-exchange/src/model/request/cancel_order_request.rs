use serde::Deserialize;
use serde::Serialize;

/// This struct represents the cancellation of an order
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CancelOrderRequest {
    pub id: String,
    pub market_pair: Option<String>,
}
