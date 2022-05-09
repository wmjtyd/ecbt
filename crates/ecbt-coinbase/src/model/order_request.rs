use super::OrderRequestType;
use super::OrderSide;
use super::OrderStop;
use serde::Deserialize;
use serde::Serialize;

/// This struct represents an order request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderRequest {
    pub side: OrderSide,
    pub client_oid: Option<String>,
    pub product_id: String,
    #[serde(flatten)]
    pub _type: OrderRequestType,
    #[serde(flatten)]
    pub stop: Option<OrderStop>,
}
