use chrono::Timelike;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

use crate::{value::repr, *};
use std::{cell::RefCell, rc::Rc};

pub struct Formatter {
  pub rc_ref_unit_manager: Rc<RefCell<UnitManager>>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct FormatOptions {
  pub precision: u32,
}

impl Default for FormatOptions {
  fn default() -> Self {
    Self { precision: 2 }
  }
}

impl Formatter {
  pub fn new(unit_manager: Rc<RefCell<UnitManager>>) -> Formatter {
    Formatter {
      rc_ref_unit_manager: unit_manager,
    }
  }

  pub fn format(&self, value: &Value, options: &FormatOptions) -> Option<String> {
    if let Some(num) = value.number() {
      let rounded_num = num.round_dp_with_strategy(
        options.precision,
        rust_decimal::RoundingStrategy::MidpointAwayFromZero,
      );
      let mut decimal_format = rounded_num.to_string();
      if value.repr() == Some(repr::SCIENTIFIC) {
        decimal_format = format!("{0:e}", rounded_num);
      }
      if value.repr() == Some(repr::BINARY) {
        if let Some(int_num) = rounded_num.trunc().to_isize() {
          decimal_format = format!("0b{0:b}", int_num);
        }
      }
      if value.repr() == Some(repr::OCTAL) {
        if let Some(int_num) = rounded_num.trunc().to_isize() {
          decimal_format = format!("0o{0:o}", int_num);
        }
      }
      if value.repr() == Some(repr::HEX) {
        if let Some(int_num) = rounded_num.trunc().to_isize() {
          decimal_format = format!("0x{0:x}", int_num);
        }
      }
      if let Some(unit_id) = value.unit() {
        if let Some(unit_format) = self.rc_ref_unit_manager.borrow().format(unit_id) {
          if unit_format.contains("%@") {
            return Some(unit_format.replace("%@", decimal_format.as_str()));
          }
          return Some(format!("{decimal_format} {unit_format}"));
        }
      }
      return Some(decimal_format);
    }
    if let Some(datetime) = value.datetime() {
      let date_string = datetime.naivedt.format("%Y-%m-%d %H:%M:%S").to_string();
      if datetime.naivedt.second() == 0
        && datetime.naivedt.minute() == 0
        && datetime.naivedt.hour() == 0
      {
        return Some(datetime.naivedt.format("%Y-%m-%d").to_string());
      }
      return Some(date_string);
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use date::DateTime;

  #[test]
  fn test_datetime() {
    let value =
      Value::default().with_datetime(DateTime::with_str_datetime("2021-01-02 01:02:03").unwrap());
    let um = UnitManager::default();
    let formatter = Formatter::new(Rc::new(RefCell::new(um)));
    assert_eq!(
      formatter.format(&value, &FormatOptions::default()),
      Some("2021-01-02 01:02:03".to_owned())
    );
  }

  #[test]
  fn test_date() {
    let value = Value::default().with_datetime(DateTime::with_str_date("2021-01-02").unwrap());
    let um = UnitManager::default();
    let formatter = Formatter::new(Rc::new(RefCell::new(um)));
    assert_eq!(
      formatter.format(&value, &FormatOptions::default()),
      Some("2021-01-02".to_owned())
    );
  }
}
