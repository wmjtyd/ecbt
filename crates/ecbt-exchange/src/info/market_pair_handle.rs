use super::shared::Result;
use super::MarketPairInfo;
use crate::errors::EcbtError;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug)]
pub struct MarketPairHandle {
    pub inner: Arc<RwLock<MarketPairInfo>>,
}

impl MarketPairHandle {
    pub fn new(inner: Arc<RwLock<MarketPairInfo>>) -> Self {
        Self { inner }
    }

    pub fn read(&self) -> Result<MarketPairInfo> {
        self.inner
            .read()
            .map(|guard| guard.clone())
            .map_err(|_| EcbtError::PoisonError())
    }
}

impl serde::Serialize for MarketPairHandle {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.inner.read().expect("Couldn't read pairs.").symbol)
    }
}
