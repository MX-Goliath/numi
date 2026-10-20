use chrono::{Local, NaiveDateTime};
use chrono_tz::Tz;

#[derive(Debug, Clone, PartialEq)]
pub struct DateTime {
  pub naivedt: NaiveDateTime,
  pub tz: Option<Tz>,
}

impl DateTime {
  pub fn new(datetime: NaiveDateTime, timezone: Option<Tz>) -> DateTime {
    DateTime {
      naivedt: datetime,
      tz: timezone,
    }
  }

  pub fn with_str_date(str: &str) -> Option<Self> {
    NaiveDateTime::parse_from_str(&format!("{} 00:00:00", str), "%Y-%m-%d %H:%M:%S")
      .ok()
      .map(|datetime| DateTime {
        naivedt: datetime,
        tz: None,
      })
  }

  pub fn with_datetime(self, datetime: NaiveDateTime) -> DateTime {
    DateTime {
      naivedt: datetime,
      tz: self.tz,
    }
  }

  pub fn with_timezone(self, timezone: Tz) -> DateTime {
    DateTime {
      naivedt: self.naivedt,
      tz: Some(timezone),
    }
  }

  pub fn with_str_datetime(str: &str) -> Option<Self> {
    let current_date = Local::now().naive_local().date();
    NaiveDateTime::parse_from_str(str, "%Y-%m-%d %H:%M:%S")
      .or_else(|_| NaiveDateTime::parse_from_str(str, "%Y-%m-%d %H:%M"))
      .or_else(|_| {
        NaiveDateTime::parse_from_str(&format!("{} {}", current_date, str), "%Y-%m-%d %H:%M")
      })
      .or_else(|_| {
        NaiveDateTime::parse_from_str(&format!("{} {}", current_date, str), "%Y-%m-%d %H:%M:%S")
      })
      .ok()
      .map(|datetime| DateTime {
        naivedt: datetime,
        tz: None,
      })
  }
}
