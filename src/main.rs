use std::fs;
use std::error::Error;
use std::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::{self, Visitor}};
use chrono::{prelude::*, ParseError};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: u64,
    uuid: String,
    subject: String,
    projects: Vec<String>,
    contexts: Vec<String>,
    due: String,
    completed: bool, 
    completed_date: DeserializableDateTime,
    status: String,
    archived: bool, 
    is_priority: bool,
    notes: Option<Vec<String>>,
    recur: String,
    recur_until: String,
    prev_recur_todo_uuid: String,
}

#[derive(Debug, Clone)]
struct DeserializableDateTime {
    date: Option<DateTime<Local>>
}
struct DateTimeVisitor;

impl<'de> Deserialize<'de> for DeserializableDateTime {
    fn deserialize<D>(deserializer: D) -> Result<DeserializableDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(DateTimeVisitor)
    }
}

impl<'de> Visitor<'de> for DateTimeVisitor {
    type Value = DeserializableDateTime;
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A valid dateTime in the format yyyy-mm-ddThh-mm-ss-oo:oo")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if s == "" {
            return Ok(DeserializableDateTime { date: None });
        }
        let localdate: Result<DateTime<Local>, ParseError> = s.parse::<DateTime<Local>>();
        match localdate {
            Ok(x) => Ok(DeserializableDateTime { date: Some(x) }),
            Err(_) => Err(E::custom(format!("invalid date time: {}", s)))
        }
    }
}

impl Serialize for DeserializableDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        match self.date {
            Some(x) => serializer.serialize_str(&x.format("%Y-%m-%dT%H:%M:%S%:z").to_string()),
            None => serializer.serialize_str("")
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let todos_raw = fs::read_to_string("/home/tortus/.todos.json")?;
    let r: Vec<Todo> = serde_json::from_str(&todos_raw)?;
    println!("{:?}", r);
    let redone = serde_json::to_string(&r)?;
    fs::write("output.json", redone)?;
    Ok(())
}
