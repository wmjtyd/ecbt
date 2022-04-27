use super::TimeInForce;
use serde::de;
use serde::de::Visitor;
use std::fmt;
use time::Duration;

/// This struct uses the time in force enum
pub struct TimeInForceVisitor;

impl<'de> Visitor<'de> for TimeInForceVisitor {
    type Value = TimeInForce;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an string, either GTC, IOC, FOK, GTT,duration")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Some(stripped) = v.strip_prefix("GTT,") {
            match stripped.parse::<u64>() {
                Ok(v) => Ok(TimeInForce::GoodTillTime(Duration::milliseconds(v as i64))),
                _ => Err(E::custom(format!("Invalid GTG: {}", v))),
            }
        } else {
            match v {
                "GTC" => Ok(TimeInForce::GoodTillCancelled),
                "IOC" => Ok(TimeInForce::ImmediateOrCancelled),
                "FOK" => Ok(TimeInForce::FillOrKill),
                _ => Err(E::custom(format!("Invalid string: {}", v))),
            }
        }
    }
}
