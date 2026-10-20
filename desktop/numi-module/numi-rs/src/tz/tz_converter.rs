use crate::{date::DateTime, eval_context::DataContext, unit_converter::ValueConverter, Value};
use chrono::{Local, Offset, TimeZone};
use chrono_tz::{Tz, TZ_VARIANTS}; // Add this import

#[derive(Clone, Default)]
pub struct TzConverter {}

impl ValueConverter for TzConverter {
  fn convert(&self, value: &Value, to: &Value, context: &DataContext) -> Option<Value> {
    let to_tz_str = to.timezone()?;
    let to_tz: Tz = match to_tz_str.parse() {
      Ok(tz) => tz,
      Err(_) => return None,
    };
    let from_date = value.datetime()?;
    let from_naivedt = from_date.naivedt;
    let from_tz = from_date.tz.unwrap_or_else(|| get_local_timezone(context));
    let source_dt = match from_tz.from_local_datetime(&from_naivedt) {
      chrono::LocalResult::Single(dt) => dt,
      _ => return None,
    };
    let target_dt = source_dt.with_timezone(&to_tz);
    let result_dt = DateTime {
      naivedt: target_dt.naive_local(),
      tz: Some(to_tz),
    };
    Some(Value::default().with_datetime(result_dt))
  }
}

fn get_local_timezone(context: &DataContext) -> Tz {
  if let Some(tz) = context.default_timezone() {
    return tz.parse().unwrap();
  }
  let local_offset = Local::now().offset().fix().local_minus_utc();

  for tz in TZ_VARIANTS {
    let offset = tz.offset_from_utc_datetime(&Local::now().naive_utc());
    if offset.fix().local_minus_utc() == local_offset {
      return tz;
    }
  }

  // Fallback to a default timezone if no match is found
  chrono_tz::America::Los_Angeles
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::NaiveDateTime;
  use chrono_tz::{America::Los_Angeles, Asia::Tokyo};

  #[test]
  fn test_timezone_conversion() {
    let converter = TzConverter::default();

    // Create source datetime in PT
    let naive_dt =
      NaiveDateTime::parse_from_str("2024-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let source_dt = DateTime::new(naive_dt, Some(Los_Angeles));
    let source_value = Value::default().with_datetime(source_dt);

    // Create target timezone (Tokyo)
    let target_value = Value::default().with_timezone("Asia/Tokyo");

    // Convert
    let result = converter
      .convert(&source_value, &target_value, &DataContext::default())
      .unwrap();
    let result_dt = result.datetime().unwrap();

    // Assert timezone
    assert_eq!(result_dt.tz, Some(Tokyo));

    // Assert time is correctly converted (PT 10:00 = Tokyo 03:00 next day)
    assert_eq!(
      result_dt.naivedt.format("%Y-%m-%d %H:%M:%S").to_string(),
      "2024-01-02 03:00:00"
    );
  }

  #[test]
  fn test_same_timezone() {
    let converter = TzConverter::default();

    let naive_dt =
      NaiveDateTime::parse_from_str("2024-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let source_dt = DateTime::new(naive_dt, Some(Los_Angeles));
    let source_value = Value::default().with_datetime(source_dt);

    let target_value = Value::default().with_timezone("America/Los_Angeles");

    let result = converter
      .convert(&source_value, &target_value, &DataContext::default())
      .unwrap();
    let result_dt = result.datetime().unwrap();

    assert_eq!(result_dt.tz, Some(Los_Angeles));
    assert_eq!(
      result_dt.naivedt.format("%Y-%m-%d %H:%M:%S").to_string(),
      "2024-01-01 10:00:00"
    );
  }
}
