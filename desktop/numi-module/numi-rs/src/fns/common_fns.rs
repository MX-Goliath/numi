use crate::fns::Func;
use num_traits::{FromPrimitive, ToPrimitive};
use rust_decimal::{Decimal, MathematicalOps};

#[derive(Default)]
pub struct CommonFns {}

impl Func for CommonFns {
  fn evaluate(
    &self,
    fn_id: &str,
    values: &[crate::Value],
    _context: &crate::eval_context::DataContext,
  ) -> Option<crate::Value> {
    let v1 = values.first()?;
    let num = v1.number()?;

    if fn_id == "sqrt" {
      if let Some(res) = num.sqrt() {
        return Some(v1.clone().with_number(res));
      }
    }

    if fn_id == "cbrt" {
      if let Some(f_num) = num.to_f64() {
        if let Some(d_res) = Decimal::from_f64(f_num.cbrt()) {
          return Some(v1.clone().with_number(d_res));
        }
      }
    }

    if fn_id == "abs" {
      return Some(v1.clone().with_number(num.abs()));
    }

    if fn_id == "log" {
      let f_num = num.to_f64()?;
      let v2 = values.get(1)?;
      let v2_num = v2.number()?;
      let v2_f_num = v2_num.to_f64()?;
      if let Some(d_res) = Decimal::from_f64(f_num.log(v2_f_num)) {
        return Some(v1.clone().with_number(d_res));
      }
    }

    if fn_id == "ln" {
      return Some(v1.clone().with_number(num.ln()));
    }

    if fn_id == "round" {
      return Some(v1.clone().with_number(num.round()));
    }

    if fn_id == "ceil" {
      return Some(v1.clone().with_number(num.ceil()));
    }

    if fn_id == "floor" {
      return Some(v1.clone().with_number(num.floor()));
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
  fn test_round() {
    let f = CommonFns::default();
    assert_eq!(
      f.evaluate(
        "round",
        &[Value::default().with_number(dec!(1.3))],
        &DataContext::default()
      )
      .unwrap(),
      Value::default().with_int(1)
    );
  }

  #[test]
  fn test_abs() {
    let f = CommonFns::default();
    assert_eq!(
      f.evaluate(
        "abs",
        &[Value::default().with_int(-1)],
        &DataContext::default()
      )
      .unwrap(),
      Value::default().with_int(1)
    );
  }
}
