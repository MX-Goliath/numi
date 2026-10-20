use crate::eval_context::DataContext;
use crate::op::BinaryOp;
use crate::unit_converter::UnitConverter;
use crate::Value;
use num_traits::ToPrimitive;
use rust_decimal::MathematicalOps;
use rust_decimal_macros::dec;
use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::Rem;
use std::rc::Rc;

#[derive(Default)]
pub struct DecimalOp {
  pub converter: Rc<RefCell<UnitConverter>>,
}

impl DecimalOp {
  pub fn new(converter: Rc<RefCell<UnitConverter>>) -> Self {
    Self { converter }
  }
}

impl BinaryOp for DecimalOp {
  fn evaluate_op(
    &self,
    op_id: &str,
    left: &Value,
    right: &Value,
    context: &DataContext,
  ) -> Option<Value> {
    let mut l_num = left.number()?;
    let mut r_num = right.number()?;

    // use result value just to carry attributes (i.e. unit), the actual return value
    // will have different number
    let mut result_value = left.clone();

    if let Some(r_unit) = right.unit() {
      if r_unit != "%" {
        result_value = right.clone();
        if let Some(conv_value) = self.converter.borrow().convert(left, right, context) {
          if let Some(conv_num) = conv_value.number() {
            l_num = conv_num;
          }
        }
      }
    }

    if op_id == "+" {
      if let Some(unit) = right.unit() {
        if unit == "%" {
          r_num = l_num * r_num / dec!(100);
        }
      }
      let result_value = result_value.with_number(l_num + r_num);
      return Some(result_value);
    }

    if op_id == "-" {
      if let Some(unit) = right.unit() {
        if unit == "%" {
          r_num = l_num * r_num / dec!(100);
        }
      }
      let result_value = result_value.with_number(l_num - r_num);
      return Some(result_value);
    }

    if op_id == "*" {
      if let Some(unit) = right.unit() {
        if unit == "%" {
          r_num /= dec!(100);
        }
      }
      let result_value = result_value.with_number(l_num * r_num);
      return Some(result_value);
    }

    if op_id == "/" {
      let result_value = result_value.with_number(l_num / r_num);
      return Some(result_value);
    }

    if op_id == "exp" {
      return Some(result_value.with_number(l_num.powd(r_num)));
    }

    if op_id == "mod" {
      return Some(result_value.with_number(l_num.rem(r_num)));
    }

    let ops_with_int = HashSet::from(["and", "or", "xor", "rshift", "lshift"]);
    if ops_with_int.contains(op_id) {
      let l_int = l_num.to_isize()?;
      let r_int = r_num.to_isize()?;
      if op_id == "and" {
        return Some(result_value.with_int(l_int & r_int));
      }
      if op_id == "or" {
        return Some(result_value.with_int(l_int | r_int));
      }
      if op_id == "xor" {
        return Some(result_value.with_int(l_int ^ r_int));
      }
      if op_id == "lshift" {
        return Some(result_value.with_int(l_int << r_int));
      }
      if op_id == "rshift" {
        return Some(result_value.with_int(l_int >> r_int));
      }
    }

    None
  }
}

#[cfg(test)]
mod decimal_binary_op_test {
  use super::DecimalOp;
  use crate::{eval_context::DataContext, op::BinaryOp, Value};

  #[test]
  fn test_decimal_sum() {
    let op = DecimalOp::default();
    let result = op.evaluate_op(
      "+",
      &Value::default().with_int(1).with_unit("a"),
      &Value::default().with_int(2),
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(3).with_unit("a"));
  }

  #[test]
  fn test_add_percent() {
    let op = DecimalOp::default();
    let mut p_value = Value::default().with_int(50);
    p_value.add_unit("%");
    let result = op.evaluate_op(
      "+",
      &Value::default().with_int(2),
      &p_value,
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(3));
  }

  #[test]
  fn test_decimal_sub() {
    let op = DecimalOp::default();
    let result = op.evaluate_op(
      "-",
      &Value::default().with_int(3),
      &Value::default().with_int(2),
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(1));
  }

  #[test]
  fn test_sub_percent() {
    let op = DecimalOp::default();
    let mut p_value = Value::default().with_int(50);
    p_value.add_unit("%");
    let result = op.evaluate_op(
      "-",
      &Value::default().with_int(2),
      &p_value,
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(1));
  }

  #[test]
  fn test_div() {
    let op = DecimalOp::default();
    let result = op.evaluate_op(
      "/",
      &Value::default().with_int(6),
      &Value::default().with_int(2),
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(3));
  }

  #[test]
  fn test_decimal_mul() {
    let op = DecimalOp::default();
    let result = op.evaluate_op(
      "*",
      &Value::default().with_int(2),
      &Value::default().with_int(3),
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(6));
  }

  #[test]
  fn test_mul_percent() {
    let op = DecimalOp::default();
    let p_value = Value::default().with_int(50).with_unit("%");
    let result = op.evaluate_op(
      "*",
      &Value::default().with_int(2),
      &p_value,
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(1));
  }

  #[test]
  fn test_exp() {
    let op = DecimalOp::default();
    let result = op.evaluate_op(
      "exp",
      &Value::default().with_int(2),
      &Value::default().with_int(3),
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(8));
  }

  #[test]
  fn test_rem() {
    let op = DecimalOp::default();
    let result = op.evaluate_op(
      "mod",
      &Value::default().with_int(5),
      &Value::default().with_int(2),
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(1));
  }

  #[test]
  fn test_bitwise_and() {
    let op = DecimalOp::default();
    let result = op.evaluate_op(
      "and",
      &Value::default().with_int(3),
      &Value::default().with_int(2),
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(2));
  }
}
