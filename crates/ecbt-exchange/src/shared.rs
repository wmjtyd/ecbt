pub type Result<T> = std::result::Result<T, crate::errors::EcbtError>;

pub mod string_to_decimal {
    use std::fmt;

    use rust_decimal::prelude::*;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringToDecimal {
            String(String),
        }

        let StringToDecimal::String(s) = StringToDecimal::deserialize(deserializer)?;
        Decimal::from_str(&s).map_err(de::Error::custom)
    }
}

pub mod string_to_opt_decimal {
    use rust_decimal::prelude::*;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<Decimal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(value) = value {
            return serializer.collect_str(&value);
        }
        serializer.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringToOptDecimal {
            String(Option<String>),
        }
        let StringToOptDecimal::String(s) = StringToOptDecimal::deserialize(deserializer)?;
        if let Some(s) = s {
            return Decimal::from_str(&s).map(Some).or(Ok(None));
        }
        Ok(None)
    }
}

pub mod iso8601_datetime_from_string {
    use serde::{Deserialize, Deserializer, Serializer};
    use time::{format_description::well_known::Rfc3339, OffsetDateTime};

    pub fn serialize<S>(value: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Ok(time) = value.format(&Rfc3339) {
            return serializer.collect_str(time.as_str());
        }
        serializer.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum DatetimeFromString {
            String(String),
        }

        let DatetimeFromString::String(s) = DatetimeFromString::deserialize(deserializer)?;
        OffsetDateTime::parse(&s, &Rfc3339).map_err(serde::de::Error::custom)
    }
}

pub mod opt_iso8601_datetime_from_string {
    use serde::{Deserialize, Deserializer, Serializer};
    use time::{format_description::well_known::Rfc3339, OffsetDateTime};

    pub fn serialize<S>(value: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(value) = value {
            if let Ok(time) = value.format(&Rfc3339) {
                return serializer.collect_str(time.as_str());
            }
        }
        serializer.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum OptDatetimeFromString {
            String(Option<String>),
        }

        let OptDatetimeFromString::String(s) = OptDatetimeFromString::deserialize(deserializer)?;
        if let Some(s) = s {
            return OffsetDateTime::parse(&s, &Rfc3339).map(Some).or(Ok(None));
        }
        Ok(None)
    }
}

pub fn utc_now() -> i64 {
    let time = time::OffsetDateTime::now_utc();
    time.unix_timestamp() * 1_000 + time.millisecond() as i64
}

pub fn timestamp_mills(time: &time::OffsetDateTime) -> i64 {
    time.unix_timestamp() * 1_000 + time.millisecond() as i64
}

pub fn timestamp_to_iso8601_datetime(timestamp: u64) -> Option<time::OffsetDateTime> {
    match time::OffsetDateTime::from_unix_timestamp_nanos(timestamp as i128 * 1_000_000) {
        Ok(time) => Some(time),
        Err(_) => None,
    }
}
