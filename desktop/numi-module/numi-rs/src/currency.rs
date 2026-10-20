use crate::{
  eval_context::DataContext, unit_converter::ValueConverter, utils::VecStringToStr, Calc, Unit,
  Value,
};
use num_traits::FromPrimitive;
use rust_decimal::Decimal;
use serde_json::{Result, Value as JsonValue};

pub const USD_ID: &str = "USD";

#[derive(Clone, Default)]
pub struct CurrencyConverter {}

impl ValueConverter for CurrencyConverter {
  fn convert(
    &self,
    value: &crate::Value,
    to: &Value,
    context: &crate::eval_context::DataContext,
  ) -> Option<crate::Value> {
    let from_unit_id = value.unit()?;
    let from_number = value.number()?;
    let to_unit_id = to.unit()?;
    let to_usd_ratio = context.value(from_unit_id).number()?;
    let to_unit_ratio = context.value(to_unit_id).number()?;
    let result_num = from_number / to_usd_ratio * to_unit_ratio;
    Some(value.clone().with_number(result_num).with_unit(to_unit_id))
  }
}

#[cfg(test)]
mod test {

  use super::*;
  use crate::{eval_context::DataContext, Value};

  #[test]
  fn test_no_rate() {
    let converter = CurrencyConverter::default();
    let value = Value::default().with_int(2).with_unit("CAD");
    assert_eq!(
      converter.convert(
        &value,
        &Value::empty().with_unit("USD"),
        &DataContext::default()
      ),
      None
    );
  }

  #[test]
  fn test_rate() {
    let converter = CurrencyConverter::default();
    let context = DataContext::default();
    context.set_value("USD", &Value::default().with_int(1));
    context.set_value("BYN", &Value::default().with_int(2));
    context.set_value("CAD", &Value::default().with_int(3));
    let value = Value::default().with_int(3).with_unit("CAD");
    assert_eq!(
      converter.convert(&value, &Value::empty().with_unit("USD"), &context),
      Some(Value::default().with_int(1).with_unit("USD"))
    );
    assert_eq!(
      converter.convert(&value, &Value::empty().with_unit("BYN"), &context),
      Some(Value::default().with_int(2).with_unit("BYN"))
    );
  }
}

pub struct CurrencyJSONParser {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrencyRate {
  currency: String,
  rate: Decimal,
}

impl CurrencyRate {
  pub fn new(currency: &str, rate: Decimal) -> Self {
    Self {
      currency: currency.to_string(),
      rate,
    }
  }
}

impl CurrencyJSONParser {
  pub fn parse(&self, json: &str) -> Vec<CurrencyRate> {
    let mut result = vec![];
    let v: Result<JsonValue> = serde_json::from_str(json);
    if let Ok(value) = v {
      if let Some(els) = value["rates"].as_object() {
        for key in els.keys() {
          if let Some(num) = els[key].as_f64() {
            if let Some(num_dec) = Decimal::from_f64(num) {
              let rate = CurrencyRate::new(key, num_dec);
              result.push(rate);
            }
          }
        }
      }
    }
    result
  }
}

#[cfg(test)]
mod test_parser {

  use super::*;
  use rust_decimal_macros::dec;

  #[test]
  fn test_empty() {
    let parser = CurrencyJSONParser {};
    let json_str = "";
    assert_eq!(parser.parse(json_str), vec![]);
  }

  #[test]
  fn test_parse() {
    let parser = CurrencyJSONParser {};
    let json_str = r#"{
			"timestamp": 1675098000,
			"base": "USD",
			"rates": {
				"AED": 1,
				"AFN": 2.5
				}
			}"#;
    assert_eq!(
      parser.parse(json_str),
      vec![
        CurrencyRate::new("AED", dec!(1)),
        CurrencyRate::new("AFN", dec!(2.5))
      ]
    );
  }
}

impl DataContext {
  pub fn fill_rates(&mut self, rates: &Vec<CurrencyRate>) {
    for rate in rates {
      self.set_value(&rate.currency, &Value::default().with_number(rate.rate));
    }
  }
}

impl Calc {
  // TODO: Consider lower priority and/or exact case matching for currency codes
  pub fn fill_rates_from_json_str(&mut self, json_str: &str) {
    let parser = CurrencyJSONParser {};
    let rates = parser.parse(json_str);
    self.data_context.fill_rates(&rates);
    for rate in rates {
      let mut variants = vec![rate.currency.clone(), rate.currency.to_lowercase()];
      let variants_key = format!("{}.variants", &rate.currency);
      variants.append(&mut self.localizer.get_strings_lo(&variants_key));
      let mut format = rate.currency.clone();
      let format_key = format!("{}.format", &rate.currency);
      if let Some(loc_format) = self.localizer.get_string(&format_key) {
        format = loc_format.to_string();
      }
      self.add_unit(Unit::new(&rate.currency, &format), &variants.to_strs());
    }
  }
}
