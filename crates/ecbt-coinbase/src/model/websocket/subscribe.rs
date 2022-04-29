use super::Auth;
use super::Channel;
use super::SubscribeCmd;
use serde::Deserialize;
use serde::Serialize;

/// This struct represents a subscribe
#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    #[serde(rename = "type")]
    pub _type: SubscribeCmd,
    pub product_ids: Vec<String>,
    pub channels: Vec<Channel>,
    #[serde(flatten)]
    pub auth: Option<Auth>,
}
