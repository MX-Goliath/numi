use crate::{eval_context::DataContext, unit_manager::UnitId, Value};
use rust_decimal::Decimal;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub trait ValueConverter {
  fn convert(&self, value: &Value, to: &Value, context: &DataContext) -> Option<Value>;
}

#[derive(Default)]
pub struct UnitConverter {
  converters: Vec<Box<dyn ValueConverter>>,
}

impl UnitConverter {
  pub fn convert(&self, value: &Value, to: &Value, context: &DataContext) -> Option<Value> {
    let result = self.iterate_over_converters(value, to, context);

    if let Some(some_result) = &result {
      // just a safety check so less surprises
      if some_result.has_unit() {
        let result2 = self.iterate_over_converters(some_result, to, context);
        if let Some(value2) = &result2 {
          // if first value has no unit or result matches required unit, then fine
          if !value.has_unit() {
            return result2;
          }
          if let Some(to_unit) = to.unit() {
            if value2.unit() == Some(to_unit) {
              return result2;
            }
          }
        }
        return None;
      }
    }

    result
  }

  fn iterate_over_converters(
    &self,
    value: &Value,
    to: &Value,
    context: &DataContext,
  ) -> Option<Value> {
    // TODO: Migrate to custom converters / optimize
    if let Some(to_unit) = to.unit() {
      // this path just for units
      if let Some(from_unit_id) = value.unit() {
        if from_unit_id == to_unit {
          return Some(value.clone());
        }
        for conv in self.converters.iter() {
          if let Some(value) = conv.convert(value, to, context) {
            return Some(value);
          }
        }
      } else {
        let mut result = value.clone();
        result.add_unit(to_unit);
        return Some(result);
      }
    } else {
      // this path just for time zone - generic
      for conv in self.converters.iter() {
        if let Some(value) = conv.convert(value, to, context) {
          return Some(value);
        }
      }
    }
    None
  }

  pub fn add_converter(&mut self, converter: Box<dyn ValueConverter>) {
    self.converters.push(converter);
  }
}

#[derive(Clone, Default)]
pub struct RatioValueConverter {
  ratios: HashMap<String, HashMap<String, Decimal>>,
}

impl ValueConverter for Rc<RefCell<RatioValueConverter>> {
  fn convert(&self, value: &Value, to: &Value, context: &DataContext) -> Option<Value> {
    self.borrow().convert(value, to, context)
  }
}

impl RatioValueConverter {
  pub fn new(from_unit_id: UnitId, to_unit_id: UnitId, ratio: Decimal) -> Self {
    let mut res = Self::default();
    res.set_ratio(&from_unit_id, &to_unit_id, ratio);
    res
  }

  pub fn set_ratio(&mut self, from_unit: &str, to_unit: &str, ratio: Decimal) {
    if let Some(from) = self.ratios.get_mut(from_unit) {
      from.insert(to_unit.to_string(), ratio);
    } else {
      let mut from: HashMap<String, Decimal> = HashMap::new();
      from.insert(to_unit.to_string(), ratio);
      self.ratios.insert(from_unit.to_string(), from);
    }
  }

  pub fn reverse_converter(&self) -> Self {
    self.clone()
  }
}

impl ValueConverter for RatioValueConverter {
  fn convert(&self, value: &Value, to: &Value, _context: &DataContext) -> Option<Value> {
    // The logic of the method right now is to use hashmap table to find direct conversion,
    // If no success then try to find inverse conversion with inversed ratio applied
    // And if it also will not succeded, then just convert to whatever it can (first match
    // from the unit). It then will try to perform same operation again, and convert
    // whatever_if_could_to to result unit. This means when specifying ratio, we should specify
    // *to the base unit*. The method can be significantly improvded with multiple rounds
    // of conversion and check for the same value result.
    if let Some(dec_value) = value.number() {
      if let Some(from_unit) = value.unit() {
        // Try to use ratio to converted unit
        if let Some(from_map) = self.ratios.get(from_unit) {
          if let Some(to_unit_id) = to.unit() {
            if let Some(ratio) = from_map.get(to_unit_id) {
              let mut result = value.clone();
              result.add_number(dec_value * ratio);
              result.add_unit(to_unit_id);
              return Some(result);
            }
          }
        }

        // Try ratio for inversed
        if let Some(to_unit_id) = to.unit() {
          if let Some(to_map) = self.ratios.get(to_unit_id) {
            if let Some(ratio) = to_map.get(from_unit) {
              let mut result = value.clone();
              result.add_number(dec_value / ratio);
              result.add_unit(to_unit_id);
              return Some(result);
            }
          }
        }

        // If no direct match provided, return whatever conversion we can do
        if let Some(from_map) = self.ratios.get(from_unit) {
          if let Some((can_to_unit_id, ratio)) = from_map.iter().next() {
            let mut result = value.clone();
            result.add_number(dec_value * ratio);
            result.add_unit(can_to_unit_id);
            return Some(result);
          }
        }
      }
    }
    None
  }
}

#[cfg(test)]
mod ratio_value_converter_test {

  use rust_decimal_macros::dec;

  use super::*;

  #[test]
  fn test_direct_ratio() {
    let mut conv = RatioValueConverter::default();
    conv.set_ratio("km", "m", dec!(1000));
    let result = conv
      .convert(
        &Value::empty().with_int(3).with_unit("km"),
        &Value::empty().with_unit("m"),
        &DataContext::default(),
      )
      .unwrap_or_default();
    assert_eq!(result, Value::default().with_int(3000).with_unit("m"));
  }

  #[test]
  fn test_reverse_ratio() {
    let mut conv = RatioValueConverter::default();
    conv.set_ratio("km", "m", dec!(1000));
    let result = conv
      .convert(
        &Value::empty().with_int(3000).with_unit("m"),
        &Value::empty().with_unit("km"),
        &DataContext::default(),
      )
      .unwrap_or_default();
    assert_eq!(result, Value::default().with_int(3).with_unit("km"));
  }

  #[test]
  fn test_fallback_to_known_unit() {
    let mut conv = RatioValueConverter::default();
    conv.set_ratio("km", "m", dec!(1000));
    let result = conv
      .convert(
        &Value::empty().with_int(3).with_unit("km"),
        &Value::empty().with_unit("cm"),
        &DataContext::default(),
      )
      .unwrap_or_default();
    assert_eq!(result, Value::default().with_int(3000).with_unit("m"));
  }
}

#[cfg(test)]
mod test {

  use super::*;
  use rust_decimal_macros::dec;

  #[test]
  fn test_simple_conversion() {
    let mut conv = RatioValueConverter::default();
    conv.set_ratio("km", "m", dec!(1000));
    let mut converter = UnitConverter::default();
    converter.add_converter(Box::new(conv));
    assert_eq!(
      converter.convert(
        &Value::default().with_int(3).with_unit("km"),
        &Value::empty().with_unit("m"),
        &DataContext::default()
      ),
      Some(Value::default().with_int(3000).with_unit("m"))
    );
  }

  #[test]
  fn test_same_unit() {
    let converter = UnitConverter::default();
    assert_eq!(
      converter.convert(
        &Value::default().with_int(1).with_unit("cm"),
        &Value::empty().with_unit("cm"),
        &DataContext::default()
      ),
      Some(Value::default().with_int(1).with_unit("cm"))
    );
  }

  #[test]
  fn test_convert_through_base() {
    let mut conv = RatioValueConverter::default();
    conv.set_ratio("cm", "m", dec!(0.01));
    conv.set_ratio("dm", "m", dec!(0.1));
    let mut converter = UnitConverter::default();
    converter.add_converter(Box::new(conv));
    assert_eq!(
      converter.convert(
        &Value::default().with_int(2).with_unit("dm"),
        &Value::empty().with_unit("cm"),
        &DataContext::default()
      ),
      Some(Value::default().with_int(20).with_unit("cm"))
    );
  }

  #[test]
  fn test_no_common_base_should_return_none() {
    let mut conv = RatioValueConverter::default();
    conv.set_ratio("km", "m", dec!(1000));
    let mut converter = UnitConverter::default();
    converter.add_converter(Box::new(conv));
    assert_eq!(
      converter.convert(
        &Value::default().with_int(200).with_unit("cm"),
        &Value::empty().with_unit("km"),
        &DataContext::default()
      ),
      None
    );
  }
}
