use serde::{Deserialize, Serialize};

pub use crate::model::currency::Currency;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct MarketPair(pub Currency, pub Currency);

impl MarketPair {
    pub fn inverse(&self) -> MarketPair {
        MarketPair(self.1.clone(), self.0.clone())
    }
}

/// to_symbol
impl ToString for MarketPair {
    fn to_string(&self) -> String {
        format!("{}{}", self.0, self.1).to_uppercase()
    }
}
