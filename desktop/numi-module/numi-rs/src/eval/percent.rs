use super::Evaluator;
use crate::{eval_context::DataContext, op::*, unit_converter::UnitConverter, Value};
use rust_decimal_macros::dec;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

pub struct PercentEval {
  binary_op_eval: BinaryOpEvaluator<PercentBinaryOp>,
}

impl PercentEval {
  pub fn new(binary_op: Rc<PercentBinaryOp>) -> Self {
    Self {
      binary_op_eval: BinaryOpEvaluator {
        matcher: Box::new(DecimalOpMatcher::with_ops(vec![
          "of", "on", "off", "aspof", "aspon", "aspoff", "pofw", "ponw", "poffw",
        ])),
        binary_op,
      },
    }
  }
}

impl Evaluator for PercentEval {
  fn evaluate<'a>(
    &self,
    elements: &crate::element_list::ElementList<'a>,
    context: &crate::eval_context::EvalContext,
  ) -> Option<crate::element_list::ElementList<'a>> {
    self.binary_op_eval.evaluate(elements, context)
  }
}

#[derive(Default)]
pub struct PercentBinaryOp {
  converter: Rc<RefCell<UnitConverter>>,
}

impl PercentBinaryOp {
  pub fn new(converter: Rc<RefCell<UnitConverter>>) -> Self {
    Self { converter }
  }
}

impl BinaryOp for PercentBinaryOp {
  fn evaluate_op(
    &self,
    op_id: &str,
    left: &crate::Value,
    right: &crate::Value,
    context: &DataContext,
  ) -> Option<crate::Value> {
    let l_num = left.number()?;
    let r_num = right.number()?;

    if op_id == "of" {
      if let Some(l_unit) = left.unit() {
        if l_unit == "%" {
          return Some(right.clone().with_number(l_num / dec!(100) * r_num));
        }
      }
    }

    if op_id == "on" {
      if let Some(l_unit) = left.unit() {
        if l_unit == "%" {
          return Some(right.clone().with_number(r_num + l_num / dec!(100) * r_num));
        }
      }
    }

    if op_id == "off" {
      if let Some(l_unit) = left.unit() {
        if l_unit == "%" {
          return Some(right.clone().with_number(r_num - l_num / dec!(100) * r_num));
        }
      }
    }

    if op_id == "pofw" {
      if let Some(l_unit) = left.unit() {
        if l_unit == "%" {
          return Some(right.clone().with_number(r_num / (l_num / dec!(100))));
        }
      }
    }

    if op_id == "ponw" {
      if let Some(l_unit) = left.unit() {
        if l_unit == "%" {
          return Some(
            right
              .clone()
              .with_number(r_num * dec!(100) / (l_num + dec!(100))),
          );
        }
      }
    }

    if op_id == "poffw" {
      if let Some(l_unit) = left.unit() {
        if l_unit == "%" {
          return Some(
            right
              .clone()
              .with_number(r_num * dec!(100) / (dec!(100) - l_num)),
          );
        }
      }
    }

    // some percentage ops require conversion
    let ops_with_conv = HashSet::from(["aspof", "aspon", "aspoff"]);
    if ops_with_conv.contains(op_id) {
      let mut r_num_conv = r_num;
      if let Some(conv_value) = self.converter.borrow().convert(right, left, context) {
        if let Some(conv_num) = conv_value.number() {
          r_num_conv = conv_num;
        }
      }

      if op_id == "aspof" {
        let res = l_num / r_num_conv * dec!(100);
        return Some(Value::default().with_number(res).with_unit("%"));
      }
      if op_id == "aspon" {
        let diff = l_num - r_num_conv;
        let res = diff / r_num_conv * dec!(100);
        return Some(Value::default().with_number(res).with_unit("%"));
      }
      if op_id == "aspoff" {
        let diff = r_num_conv - l_num;
        let res = diff / r_num_conv * dec!(100);
        return Some(Value::default().with_number(res).with_unit("%"));
      }
    }
    None
  }
}

#[cfg(test)]
mod test {

  use crate::{unit_converter::RatioValueConverter, Value};

  use super::*;

  #[test]
  fn test_percent_of() {
    let percent_op = PercentBinaryOp::default();
    let result = percent_op.evaluate_op(
      "of",
      &Value::default().with_int(50).with_unit("%"),
      &Value::default().with_int(2),
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(1));
  }

  #[test]
  fn test_percent_on() {
    let op = PercentBinaryOp::default();
    let result = op.evaluate_op(
      "on",
      &Value::default().with_int(50).with_unit("%"),
      &Value::default().with_int(2),
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(3));
  }

  #[test]
  fn test_percent_off() {
    let op = PercentBinaryOp::default();
    let result = op.evaluate_op(
      "off",
      &Value::default().with_int(25).with_unit("%"),
      &Value::default().with_int(4),
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(3));
  }

  #[test]
  fn test_as_a_percent_of() {
    let op = PercentBinaryOp::default();
    let result = op.evaluate_op(
      "aspof",
      &Value::default().with_int(1),
      &Value::default().with_int(4),
      &DataContext::default(),
    );
    assert_eq!(
      result.unwrap(),
      Value::default().with_int(25).with_unit("%")
    );
  }

  #[test]
  fn test_as_a_percent_of_with_conv() {
    let mut rconv = RatioValueConverter::default();
    rconv.set_ratio("m", "cm", dec!(100));
    let mut conv = UnitConverter::default();
    conv.add_converter(Box::new(rconv));
    let op = PercentBinaryOp::new(Rc::new(RefCell::new(conv)));
    let result = op.evaluate_op(
      "aspof",
      &Value::default().with_int(5).with_unit("cm"),
      &Value::default().with_int(1).with_unit("m"),
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(5).with_unit("%"));
  }

  #[test]
  fn test_as_a_percent_on() {
    let op = PercentBinaryOp::default();
    let result = op.evaluate_op(
      "aspon",
      &Value::default().with_int(3),
      &Value::default().with_int(2),
      &DataContext::default(),
    );
    assert_eq!(
      result.unwrap(),
      Value::default().with_int(50).with_unit("%")
    );
  }

  #[test]
  fn test_as_a_percent_off() {
    let op = PercentBinaryOp::default();
    let result = op.evaluate_op(
      "aspoff",
      &Value::default().with_int(1),
      &Value::default().with_int(2),
      &DataContext::default(),
    );
    assert_eq!(
      result.unwrap(),
      Value::default().with_int(50).with_unit("%")
    );
  }

  #[test]
  fn test_percent_of_what_is() {
    let op = PercentBinaryOp::default();
    let result = op.evaluate_op(
      "pofw",
      &Value::default().with_int(50).with_unit("%"),
      &Value::default().with_int(1),
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(2));
  }

  #[test]
  fn test_percent_on_what_is() {
    let op = PercentBinaryOp::default();
    let result = op.evaluate_op(
      "ponw",
      &Value::default().with_int(50).with_unit("%"),
      &Value::default().with_int(3),
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(2));
  }

  #[test]
  fn test_percent_off_what_is() {
    let op = PercentBinaryOp::default();
    let result = op.evaluate_op(
      "poffw",
      &Value::default().with_int(50).with_unit("%"),
      &Value::default().with_int(1),
      &DataContext::default(),
    );
    assert_eq!(result.unwrap(), Value::default().with_int(2));
  }
}
