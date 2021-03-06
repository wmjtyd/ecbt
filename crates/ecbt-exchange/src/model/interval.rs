use std::convert::TryFrom;

use serde::Deserialize;
use serde::Serialize;
use time::Duration;

use crate::{EcbtError, Result};

/// This enum represents a time interval
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Interval {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "3m")]
    ThreeMinutes,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "2h")]
    TwoHours,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "6h")]
    SixHours,
    #[serde(rename = "8h")]
    EightHours,
    #[serde(rename = "12h")]
    TwelveHours,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "3d")]
    ThreeDays,
    #[serde(rename = "1w")]
    OneWeek,
    #[serde(rename = "1mo")]
    OneMonth,
}

impl From<Interval> for Duration {
    fn from(i: Interval) -> Self {
        match i {
            Interval::OneMinute => Duration::minutes(1),
            Interval::ThreeMinutes => Duration::minutes(3),
            Interval::FiveMinutes => Duration::minutes(5),
            Interval::FifteenMinutes => Duration::minutes(15),
            Interval::ThirtyMinutes => Duration::minutes(30),
            Interval::OneHour => Duration::hours(1),
            Interval::TwoHours => Duration::hours(2),
            Interval::FourHours => Duration::hours(4),
            Interval::SixHours => Duration::hours(6),
            Interval::EightHours => Duration::hours(8),
            Interval::TwelveHours => Duration::hours(12),
            Interval::OneDay => Duration::days(1),
            Interval::ThreeDays => Duration::days(3),
            Interval::OneWeek => Duration::weeks(1),
            Interval::OneMonth => Duration::days(30),
        }
    }
}

impl Interval {
    pub fn to_duration(self) -> Duration {
        self.into()
    }
}

impl TryFrom<Interval> for u32 {
    type Error = EcbtError;
    fn try_from(value: Interval) -> Result<Self> {
        match value {
            Interval::OneMinute => Ok(60),
            Interval::FiveMinutes => Ok(300),
            Interval::FifteenMinutes => Ok(900),
            Interval::OneHour => Ok(3600),
            Interval::SixHours => Ok(21600),
            Interval::OneDay => Ok(86400),
            _ => Err(EcbtError::MissingParameter(format!(
                "{:?} is not supported in Coinbase",
                value,
            ))),
        }
    }
}

impl From<Interval> for &str {
    fn from(interval: Interval) -> Self {
        match interval {
            Interval::OneMinute => "1m",
            Interval::ThreeMinutes => "3m",
            Interval::FiveMinutes => "5m",
            Interval::FifteenMinutes => "15m",
            Interval::ThirtyMinutes => "30m",
            Interval::OneHour => "1h",
            Interval::TwoHours => "2h",
            Interval::FourHours => "4h",
            Interval::SixHours => "6h",
            Interval::EightHours => "8h",
            Interval::TwelveHours => "12h",
            Interval::OneDay => "1d",
            Interval::ThreeDays => "3d",
            Interval::OneWeek => "1w",
            Interval::OneMonth => "1M",
        }
    }
}
