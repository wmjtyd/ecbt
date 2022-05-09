//! This module is used to make calls to api and connect to the websockets

mod account;
mod base_client;
mod market;
pub mod stream;

pub use super::shared;
pub(crate) use super::transport::Transport;
pub use base_client::BaseClient;
