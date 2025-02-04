use std::slice::Iter;

use chrono::{DateTime, Local, Utc};

pub enum DateTimeConverter {
    ISO8601,
    ISO9075,
    RFC3339,
    RFC7231,
    Unix,
    Timestamp,
    UTC,
}

impl DateTimeConverter {
    pub fn iter() -> Iter<'static, DateTimeConverter> {
        static CONVERTER: [DateTimeConverter; 7] = [
            DateTimeConverter::ISO8601,
            DateTimeConverter::ISO9075,
            DateTimeConverter::RFC3339,
            DateTimeConverter::RFC7231,
            DateTimeConverter::Unix,
            DateTimeConverter::Timestamp,
            DateTimeConverter::UTC,
        ];
        CONVERTER.iter()
    }

    pub fn name(&self) -> String {
        match self {
            DateTimeConverter::ISO8601 => String::from("ISO 8601"),
            DateTimeConverter::ISO9075 => String::from("ISO 9075"),
            DateTimeConverter::RFC3339 => String::from("RFC 3339"),
            DateTimeConverter::RFC7231 => String::from("RFC 7231"),
            DateTimeConverter::Unix => String::from("Unix timestamp"),
            DateTimeConverter::Timestamp => String::from("Timestamp"),
            DateTimeConverter::UTC => String::from("UTC format"),
        }
    }

    pub fn formatter(&self, datetime: DateTime<Local>) -> String {
        match self {
            DateTimeConverter::ISO8601 => datetime.format("%Y-%m-%dT%H:%M:%S%:z").to_string(),
            DateTimeConverter::ISO9075 => datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            DateTimeConverter::RFC3339 => datetime.format("%Y-%m-%dT%H:%M:%S%:z").to_string(),
            DateTimeConverter::RFC7231 => datetime
                .with_timezone(&Utc)
                .format("%a, %d %b %Y %H:%M:%S GMT")
                .to_string(),
            DateTimeConverter::Unix => datetime.timestamp().to_string(),
            DateTimeConverter::Timestamp => format!(
                "{}{:03}",
                datetime.timestamp(),
                datetime.timestamp_subsec_millis()
            ),
            DateTimeConverter::UTC => datetime
                .with_timezone(&Utc)
                .format("%a, %d %b %Y %H:%M:%S GMT")
                .to_string(),
        }
    }
}
