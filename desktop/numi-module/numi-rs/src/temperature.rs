use rust_decimal_macros::dec;

use crate::{localizer::Localizer, unit_converter::ValueConverter, Calc, Unit, Value};

#[derive(Clone, Default)]
pub struct TemperatureConverter {}

impl ValueConverter for TemperatureConverter {
  fn convert(
    &self,
    value: &Value,
    to: &Value,
    _context: &crate::eval_context::DataContext,
  ) -> Option<crate::Value> {
    let from_unit = value.unit()?;
    let from_num = value.number()?;
    let to_unit_id = to.unit()?;
    let supported_units = ["kelvin", "celsius", "fahrenheit"];
    if !supported_units.contains(&from_unit) || !supported_units.contains(&to_unit_id) {
      return None;
    }

    let celsius_kelvin_offset = dec!(273.15);
    let fahrenheit_kelvin_offset = dec!(459.67);

    let mut from_num_kelvin = from_num;

    if from_unit == "celsius" {
      from_num_kelvin = from_num + celsius_kelvin_offset;
    }

    if from_unit == "fahrenheit" {
      from_num_kelvin = (from_num + fahrenheit_kelvin_offset) * dec!(5) / dec!(9);
    }

    let mut result_num = from_num_kelvin;

    if to_unit_id == "celsius" {
      result_num -= celsius_kelvin_offset;
    }

    if to_unit_id == "fahrenheit" {
      result_num = result_num * dec!(9) / dec!(5) - fahrenheit_kelvin_offset;
    }

    let mut result = value.clone();
    result.add_number(result_num);
    result.add_unit(to_unit_id);
    Some(result)
  }
}

impl Calc {
  pub fn add_temperature(&self, localizer: &Localizer) {
    self.add_unit(
      Unit::new("celsius", localizer.get_string("celsius.format").unwrap()),
      &localizer.get_strings("celsius.variants"),
    );
    self.add_unit(
      Unit::new(
        "fahrenheit",
        localizer.get_string("fahrenheit.format").unwrap(),
      ),
      &localizer.get_strings("fahrenheit.variants"),
    );
    self.add_unit(
      Unit::new("kelvin", localizer.get_string("kelvin.format").unwrap()),
      &localizer.get_strings("kelvin.variants"),
    );
    self.add_converter(Box::<TemperatureConverter>::default());
  }
}

#[cfg(test)]
mod test {

  use crate::{eval_context::DataContext, Value};

  use super::*;

  #[test]
  fn test_celsius_to_fahrenheit() {
    let conv = TemperatureConverter::default();
    let result = conv.convert(
      &Value::default().with_int(10).with_unit("celsius"),
      &Value::default().with_unit("fahrenheit"),
      &DataContext::default(),
    );
    assert_eq!(
      result.unwrap(),
      Value::default().with_int(50).with_unit("fahrenheit")
    );
  }

  #[test]
  fn test_fahrenheit_to_celsius() {
    let conv = TemperatureConverter::default();
    let result = conv.convert(
      &Value::default().with_int(50).with_unit("fahrenheit"),
      &Value::default().with_unit("celsius"),
      &DataContext::default(),
    );
    assert_eq!(
      result.unwrap(),
      Value::default().with_int(10).with_unit("celsius")
    );
  }
}
