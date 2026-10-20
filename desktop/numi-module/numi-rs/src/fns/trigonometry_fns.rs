use num_traits::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;

use crate::fns::Func;

#[derive(Default)]
pub struct TrigonometryFns {}

impl Func for TrigonometryFns {
  fn evaluate(
    &self,
    fn_id: &str,
    values: &[crate::Value],
    _context: &crate::eval_context::DataContext,
  ) -> Option<crate::Value> {
    let v1 = values.first()?;
    let num = v1.number()?;
    let f_num = num.to_f64()?;

    if fn_id == "sin" {
      if let Some(dec) = Decimal::from_f64(f_num.sin()) {
        return Some(v1.clone().with_number(dec));
      }
    }

    if fn_id == "asin" {
      if let Some(dec) = Decimal::from_f64(f_num.asin()) {
        return Some(v1.clone().with_number(dec));
      }
    }

    if fn_id == "sinh" {
      if let Some(dec) = Decimal::from_f64(f_num.sinh()) {
        return Some(v1.clone().with_number(dec));
      }
    }

    if fn_id == "cos" {
      if let Some(dec) = Decimal::from_f64(f_num.cos()) {
        return Some(v1.clone().with_number(dec));
      }
    }

    if fn_id == "acos" {
      if let Some(dec) = Decimal::from_f64(f_num.acos()) {
        return Some(v1.clone().with_number(dec));
      }
    }

    if fn_id == "cosh" {
      if let Some(dec) = Decimal::from_f64(f_num.cosh()) {
        return Some(v1.clone().with_number(dec));
      }
    }

    if fn_id == "tan" {
      if let Some(dec) = Decimal::from_f64(f_num.tan()) {
        return Some(v1.clone().with_number(dec));
      }
    }

    if fn_id == "atan" {
      if let Some(dec) = Decimal::from_f64(f_num.atan()) {
        return Some(v1.clone().with_number(dec));
      }
    }

    if fn_id == "tanh" {
      if let Some(dec) = Decimal::from_f64(f_num.tanh()) {
        return Some(v1.clone().with_number(dec));
      }
    }

    None
  }
}

#[cfg(test)]
mod test {

  use super::*;
  use crate::{eval_context::DataContext, Value};
  use rust_decimal_macros::dec;

  #[test]
  fn test_sin() {
    let f = TrigonometryFns::default();
    assert_eq!(
      f.evaluate(
        "sin",
        &[Value::default().with_number(dec!(3.14159))],
        &DataContext::default()
      )
      .unwrap()
      .rounded(2),
      Value::default().with_int(0)
    );
  }
}
