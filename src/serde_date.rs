use serde::{Deserialize, Deserializer, Serialize, Serializer, de::{self, Visitor}};
use std::fmt;
use std::cmp::Ordering;
use chrono::{Days, format::{parse, Parsed, Numeric, Fixed, Item, Pad}, prelude::*, ParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerdeDate {
    date: Option<NaiveDate>
}
struct DateVisitor;

impl<'de> Deserialize<'de> for SerdeDate {
    fn deserialize<D>(deserializer: D) -> Result<SerdeDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(DateVisitor)
    }
}

impl<'de> Visitor<'de> for DateVisitor {
    type Value = SerdeDate;
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A valid dateTime in the format yyyy-mm-dd")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if s == "" {
            return Ok(SerdeDate{ date: None });
        }
        let localdate: Result<NaiveDate, ParseError> = s.parse::<NaiveDate>();
        match localdate {
            Ok(x) => Ok(SerdeDate { date: Some(x) }),
            Err(_) => Err(E::custom(format!("invalid date time: {}", s)))
        }
    }
}

impl Serialize for SerdeDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        match self.date {
            Some(x) => serializer.serialize_str(&x.format("%Y-%m-%d").to_string()),
            None => serializer.serialize_str("")
        }
    }
}

impl fmt::Display for SerdeDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.date {
            None => write!(f, "{}", ""),
            Some(d) => write!(f, "{}", d.format("%a %b %d").to_string()),
        }
    }
}

const PARSE_OPTIONS: [&[Item]; 3] = [
    &[Item::Fixed(Fixed::ShortMonthName), Item::Numeric(Numeric::Day, Pad::Zero)],
    &[Item::Fixed(Fixed::ShortWeekdayName)],
    &[Item::Numeric(Numeric::Year, Pad::Zero), Item::Literal("-"), Item::Numeric(Numeric::Month, Pad::Zero), Item::Literal("-"), Item::Numeric(Numeric::Day, Pad::Zero)]
];
impl TryFrom<Option<String>> for SerdeDate {
    type Error = ParseError;
    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        match value {
            None => Ok(SerdeDate { date: None }),
            Some(d) => {
                let today = Local::now().date_naive();
                let parsed_date = match d.as_str() {
                    "today" => today,
                    "tod" => today,
                    "tomorrow" => today + Days::new(1),
                    "tom" => today + Days::new(1),
                    _ => {
                        let mut parsed = Parsed::new();
                        PARSE_OPTIONS.iter().fold(
                            parse(&mut parsed, &d, PARSE_OPTIONS[0].iter()), 
                            |res, opt| res.or(parse(&mut parsed, &d, opt.iter()))
                        )?;
                        if let Some(weekday) = parsed.weekday() {
                            let diff = weekday.days_since(today.weekday());
                            return Ok(SerdeDate { date: Some(today + Days::new(diff.into()))});
                        }
                        let _ = parsed.set_year((today.year() + 
                            if today.month() * 31 + today.day() < 
                            parsed.month().unwrap() * 31 + parsed.day().unwrap() 
                                { 0 } else { 1 }).into());
                        parsed.to_naive_date()?
                    }
                };
                Ok(SerdeDate { date: Some(parsed_date) })
            },
        }
    }
}

impl SerdeDate {
    pub fn is_some(&self) -> bool {
        self.date.is_some()
    }

    pub fn today() -> Self {
        SerdeDate {
            date: Some(Local::now().date_naive())
        }
    }

    pub fn cmp(&self, other: &SerdeDate) -> Ordering {
        if let (Some(this_date_i), Some(other_date_i)) = (self.date, other.date) {
            return this_date_i.cmp(&other_date_i);
        } else {
            if self.date.is_none() && other.date.is_none() { 
                return Ordering::Equal;
            }
            if self.date.is_none() { Ordering::Greater } else { Ordering::Less }
        }
    }
}
