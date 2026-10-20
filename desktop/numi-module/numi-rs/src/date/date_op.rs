use chrono::Duration;

use crate::{eval_context::DataContext, op::BinaryOp, unit_converter::UnitConverter, Value};
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
pub struct DateOp {
  pub converter: Rc<RefCell<UnitConverter>>,
}

impl DateOp {
  pub fn new(converter: Rc<RefCell<UnitConverter>>) -> Self {
    Self { converter }
  }
}

impl BinaryOp for DateOp {
  fn evaluate_op(
    &self,
    op_id: &str,
    left: &Value,
    right: &Value,
    context: &DataContext,
  ) -> Option<Value> {
    let l_date = left.datetime()?;

    if let Some(sec_time) =
      self
        .converter
        .borrow()
        .convert(right, &Value::empty().with_unit("sec"), context)
    {
      if op_id == "+" {
        let result_datetime = l_date
          .naivedt
          .checked_add_signed(Duration::seconds(sec_time.int().unwrap() as i64));
        let result_flexdatetime = l_date.with_datetime(result_datetime.unwrap());
        return Some(Value::default().with_datetime(result_flexdatetime));
      }
      if op_id == "-" {
        let result_datetime = l_date
          .naivedt
          .checked_sub_signed(Duration::seconds(sec_time.int().unwrap() as i64));
        let result_flexdatetime = l_date.with_datetime(result_datetime.unwrap());
        return Some(Value::default().with_datetime(result_flexdatetime));
      }
    }

    None
  }
}

#[cfg(test)]
mod date_op_test {
  use crate::{
    date::{date_op::DateOp, DateTime},
    op::BinaryOp,
    Value,
  };

  #[test]
  fn test_time_add_sec() {
    let flex_time = DateTime::with_str_date("2023-03-01").unwrap();
    let l_value = Value::default().with_datetime(flex_time);
    let r_value = Value::default().with_int(10).with_unit("sec");
    let expected_flex_time = DateTime::with_str_datetime("2023-03-01 00:00:10").unwrap();
    let op = DateOp::default();
    let result = op.evaluate_op("+", &l_value, &r_value, &Default::default());
    assert_eq!(
      result.unwrap(),
      Value::default().with_datetime(expected_flex_time)
    );
  }

  #[test]
  fn test_time_substr() {
    let flex_time = DateTime::with_str_datetime("2023-03-04 12:30:40").unwrap();
    let l_value = Value::default().with_datetime(flex_time);
    let r_value = Value::default().with_int(5).with_unit("sec");
    let expected_flex_time = DateTime::with_str_datetime("2023-03-04 12:30:35").unwrap();
    let op = DateOp::default();
    let result = op.evaluate_op("-", &l_value, &r_value, &Default::default());
    assert_eq!(
      result.unwrap(),
      Value::default().with_datetime(expected_flex_time)
    );
  }
}
