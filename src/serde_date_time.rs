use serde::{Deserialize, Deserializer, Serialize, Serializer, de::{self, Visitor}};
use std::fmt;
use chrono::{prelude::*, ParseError};

#[derive(Debug, Clone)]
pub struct SerdeDateTime {
    date: Option<DateTime<Local>>
}
struct DateTimeVisitor;

impl<'de> Deserialize<'de> for SerdeDateTime {
    fn deserialize<D>(deserializer: D) -> Result<SerdeDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(DateTimeVisitor)
    }
}

impl<'de> Visitor<'de> for DateTimeVisitor {
    type Value = SerdeDateTime;
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A valid dateTime in the format yyyy-mm-ddThh-mm-ss-oo:oo")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if s == "" {
            return Ok(SerdeDateTime { date: None });
        }
        let localdate: Result<DateTime<Local>, ParseError> = s.parse::<DateTime<Local>>();
        match localdate {
            Ok(x) => Ok(SerdeDateTime { date: Some(x) }),
            Err(_) => Err(E::custom(format!("invalid date time: {}", s)))
        }
    }
}

impl Serialize for SerdeDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        match self.date {
            Some(x) => serializer.serialize_str(&x.format("%Y-%m-%dT%H:%M:%S%:z").to_string()),
            None => serializer.serialize_str("")
        }
    }
}

impl SerdeDateTime {
    pub fn new_empty() -> Self {
        SerdeDateTime {
            date: None
        }
    }

    pub fn now() -> Self {
        SerdeDateTime {
            date: Some(Local::now())
        }
    }
}