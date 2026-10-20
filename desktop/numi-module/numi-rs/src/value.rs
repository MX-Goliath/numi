use num_traits::*;
use rust_decimal::*;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::date::DateTime;

/// Stores limited amount of supported types in a dictionary
/// and provides high-level interfaces to operate on those attributes
/// Should use `Decimal` for all calculation values and use other for
/// additional ones like priority etc.
///
/// Some attributes can be combined, for example decimal value and priority
#[derive(PartialEq, Clone, Default)]
pub struct Value {
  attributes: HashMap<ValueKey, ValueType>,
}

impl Debug for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut s = f.debug_struct("Value");
    if let Some(number) = self.number() {
      s.field("number", &number);
    }
    if let Some(unit) = self.unit() {
      s.field("unit", &unit);
    }
    if let Some(op) = self.op() {
      s.field("op", &op);
    }
    if self.priority() != 0 {
      s.field("priority", &self.priority());
    }
    if let Some(datetime) = self.datetime() {
      s.field("datetime", &datetime);
    }
    if let Some(tz) = self.timezone() {
      s.field("tz", &tz);
    }
    s.finish()
  }
}

type ValueKey = i16;

/// List of supported attribute identifiers
mod value_keys {

  use super::*;

  pub const NUMBER: ValueKey = 2;
  pub const OP: ValueKey = 20;
  pub const PRIORITY: ValueKey = 30;
  pub const UNIT: ValueKey = 10;
  pub const FN: ValueKey = 25;
  pub const SCALE: ValueKey = 35;
  pub const REPR: ValueKey = 40;
  pub const DATETIME: ValueKey = 45;
  pub const TIMEZONE: ValueKey = 47;
  pub const ERROR: ValueKey = 50;
}

pub type ReprType = isize;

pub mod repr {
  use super::*;

  pub const DECIMAL: ReprType = 0;
  pub const SCIENTIFIC: ReprType = 1;
  pub const BINARY: ReprType = 2;
  pub const HEX: ReprType = 3;
  pub const OCTAL: ReprType = 4;
}

/// Enumerates supported attribute types
#[derive(PartialEq, Debug, Clone)]
enum ValueType {
  String(String),
  Int(isize),
  Char(char),
  Decimal(Decimal),
  DateTime(DateTime),
  Error,
}

// Basic methods

impl Value {
  pub fn empty() -> Value {
    Value {
      attributes: HashMap::new(),
    }
  }

  pub fn has_attributes(&self) -> bool {
    !self.attributes.is_empty()
  }

  pub fn is_empty(&self) -> bool {
    !self.has_attributes()
  }
}

// Number operations

impl Value {
  pub fn with_int(self, int: isize) -> Value {
    self.with_number(Decimal::new(int as i64, 0))
  }

  pub fn rounded(mut self, dp: u32) -> Value {
    if let Some(num) = self.number() {
      self.add_number(num.round_dp(dp));
    }
    self
  }

  pub fn int(&self) -> Option<isize> {
    if let Some(dec) = self.number() {
      if dec.fract().is_zero() {
        return dec.to_isize();
      }
    }
    None
  }

  pub fn with_number(mut self, decimal: Decimal) -> Value {
    self.add_number(decimal);
    self
  }

  pub fn add_number(&mut self, decimal: Decimal) {
    self.add_attribute(value_keys::NUMBER, ValueType::Decimal(decimal))
  }

  pub fn remove_number(&mut self) {
    self.remove_attribute(value_keys::NUMBER);
  }

  pub fn number(&self) -> Option<Decimal> {
    if let Some(ValueType::Decimal(value)) = self.attributes.get(&value_keys::NUMBER) {
      return Option::Some(*value);
    }
    Option::None
  }

  pub fn is_number(&self) -> bool {
    self.number().is_some()
  }
}

// Error

impl Value {
  pub fn with_error() -> Value {
    Value::new(value_keys::ERROR, ValueType::Error)
  }

  pub fn has_error(&self) -> bool {
    if let Some(ValueType::Error) = self.attributes.get(&value_keys::ERROR) {
      return true;
    }
    false
  }
}

// Datetime

impl Value {
  pub fn with_datetime(self, datetime: DateTime) -> Value {
    self.with_attribute(value_keys::DATETIME, ValueType::DateTime(datetime))
  }

  pub fn datetime(&self) -> Option<DateTime> {
    if let Some(ValueType::DateTime(datetime)) = self.attributes.get(&value_keys::DATETIME) {
      return Option::Some(datetime.clone());
    }
    None
  }

  pub fn is_datetime(&self) -> bool {
    self.datetime().is_some()
  }

  pub fn remove_datetime(&mut self) {
    self.remove_attribute(value_keys::DATETIME);
  }
}

// Timezone

impl Value {
  pub fn with_timezone(self, timezone: &str) -> Value {
    self.with_attribute(
      value_keys::TIMEZONE,
      ValueType::String(timezone.to_string()),
    )
  }

  pub fn timezone(&self) -> Option<&str> {
    if let Some(ValueType::String(string)) = self.attributes.get(&value_keys::TIMEZONE) {
      return Option::Some(string.as_str());
    }
    None
  }

  pub fn is_timezone(&self) -> bool {
    self.timezone().is_some()
  }

  pub fn remove_timezone(&mut self) {
    self.remove_attribute(value_keys::TIMEZONE);
  }
}

// Units

impl Value {
  pub fn with_unit(self, unit: &str) -> Value {
    self.with_attribute(value_keys::UNIT, ValueType::String(unit.to_string()))
  }

  pub fn unit(&self) -> Option<&str> {
    if let Some(ValueType::String(string)) = self.attributes.get(&value_keys::UNIT) {
      return Option::Some(string.as_str());
    }
    None
  }

  pub fn add_unit(&mut self, unit: &str) {
    self.add_attribute(value_keys::UNIT, ValueType::String(unit.to_string()));
  }

  pub fn has_unit(&self) -> bool {
    self.unit().is_some()
  }
}

// Scales

impl Value {
  pub fn with_scale(self, scale: &str) -> Value {
    self.with_attribute(value_keys::SCALE, ValueType::String(scale.to_string()))
  }

  pub fn scale(&self) -> Option<&str> {
    if let Some(ValueType::String(string)) = self.attributes.get(&value_keys::SCALE) {
      return Option::Some(string.as_str());
    }
    None
  }

  pub fn add_scale(&mut self, scale: &str) {
    self.add_attribute(value_keys::SCALE, ValueType::String(scale.to_string()));
  }

  pub fn has_scale(&self) -> bool {
    self.scale().is_some()
  }
}

// Functions

impl Value {
  pub fn with_func(self, func_id: &str) -> Value {
    self.with_attribute(value_keys::FN, ValueType::String(func_id.to_string()))
  }

  pub fn func(&self) -> Option<&str> {
    if let Some(ValueType::String(string)) = self.attributes.get(&value_keys::FN) {
      return Option::Some(string.as_str());
    }
    None
  }

  pub fn add_func(&mut self, func_id: &str) {
    self.add_attribute(value_keys::FN, ValueType::String(func_id.to_string()));
  }

  pub fn has_func(&self) -> bool {
    self.func().is_some()
  }
}

// Operation attribute support

impl Value {
  pub fn with_op(self, op: &str) -> Value {
    self.with_attribute(value_keys::OP, ValueType::String(op.to_string()))
  }

  pub fn op(&self) -> Option<&str> {
    if let Some(ValueType::String(string)) = self.attributes.get(&value_keys::OP) {
      return Option::Some(string.as_str());
    }
    None
  }

  pub fn has_op(&self) -> bool {
    self.op().is_some()
  }
}

// Priority support

impl Value {
  pub fn with_priority(mut self, priority: isize) -> Value {
    self.add_priority(priority);
    self
  }

  pub fn add_priority(&mut self, priority: isize) {
    self.add_attribute(value_keys::PRIORITY, ValueType::Int(priority));
  }

  pub fn remove_priority(&mut self) {
    self.remove_attribute(value_keys::PRIORITY);
  }

  pub fn priority(&self) -> isize {
    if let Some(ValueType::Int(value)) = self.attributes.get(&value_keys::PRIORITY) {
      return *value;
    }
    0
  }
}

// Representation

impl Value {
  pub fn with_repr(self, repr: ReprType) -> Value {
    self.with_attribute(value_keys::REPR, ValueType::Int(repr))
  }

  pub fn repr(&self) -> Option<ReprType> {
    if let Some(ValueType::Int(repr)) = self.attributes.get(&value_keys::REPR) {
      return Option::Some(*repr);
    }
    None
  }

  pub fn add_repr(&mut self, repr: ReprType) {
    self.add_attribute(value_keys::REPR, ValueType::Int(repr));
  }

  pub fn has_repr(&self) -> bool {
    self.repr().is_some()
  }
}

// Generic attributes

impl Value {
  fn new(key: ValueKey, value: ValueType) -> Value {
    let mut result = Value::empty();
    result.attributes.insert(key, value);
    result
  }

  fn add_attribute(&mut self, key: ValueKey, value: ValueType) {
    self.attributes.insert(key, value);
  }

  fn with_attribute(mut self, key: ValueKey, value: ValueType) -> Self {
    self.attributes.insert(key, value);
    self
  }

  fn without_attribute(mut self, key: ValueKey) -> Self {
    self.remove_attribute(key);
    self
  }

  fn remove_attribute(&mut self, key: ValueKey) {
    self.attributes.remove(&key);
  }

  fn has_attribute(&self, key: ValueKey) -> bool {
    self.attributes.contains_key(&key)
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn empty_has_no_attributes() {
    let value = Value::empty();
    assert!(!value.has_attributes());
  }

  #[test]
  fn test_number() {
    let value = Value::default().with_int(10);
    assert_eq!(value.int().unwrap(), 10);
  }

  #[test]
  fn test_empty() {
    let value = Value::empty();
    assert!(value.is_empty());
    assert!(!value.has_attributes());
  }

  #[test]
  fn test_debug() {
    let value = Value::default().with_int(5);
    assert_eq!(format!("{:?}", &value), "Value { number: 5 }");
  }
}
