use std::rc::Rc;

use crate::op::BinaryOp;
use crate::ElementList;
use crate::EvalContext;
use crate::Evaluator;
use crate::Value;

pub struct TokenEval {
  sum_op: Rc<dyn BinaryOp>,
}

const PREV_TOKEN_KEY: &str = "prev";
const SUM_TOKEN_KEY: &str = "sum";
const AVG_TOKEN_KEY: &str = "avg";
const CNT_TOKEN_KEY: &str = "cnt";

impl TokenEval {
  pub fn new(sum_op: Rc<dyn BinaryOp>) -> Self {
    Self { sum_op }
  }
}

impl Evaluator for TokenEval {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    if context.data.eval_level() <= 1 {
      if let Some(el) = elements.first() {
        // prev
        context.data.set_variable(PREV_TOKEN_KEY, &el.value);

        // sum
        let prev_sum = context.data.get_variable(SUM_TOKEN_KEY);
        let mut sum = el.value.clone();
        if !prev_sum.is_empty() {
          sum = self
            .sum_op
            .evaluate_op("+", &prev_sum, &el.value, &context.data)
            .unwrap_or_default();
        }
        context.data.set_variable(SUM_TOKEN_KEY, &sum);

        // count
        let cnt_val = context.data.get_variable(CNT_TOKEN_KEY);
        let counter = cnt_val.int().unwrap_or(0) + 1;
        let new_cnt_val = Value::default().with_int(counter);
        context.data.set_variable(CNT_TOKEN_KEY, &new_cnt_val);

        // avg
        let avg = self
          .sum_op
          .evaluate_op("/", &sum, &new_cnt_val, &context.data)
          .unwrap_or_default();
        context.data.set_variable(AVG_TOKEN_KEY, &avg);
      }
    }
    None
  }
}

#[cfg(test)]
mod toke_eval_test {
  use crate::{op::DecimalOp, Value};

  use super::*;
  use rust_decimal_macros::dec;

  #[test]
  fn test_prev() {
    let eval = TokenEval::new(Rc::new(DecimalOp::default()));
    let eval_context = EvalContext::dumb();
    let el_list = ElementList::with_decimal(dec!(1), "1");
    let _ = eval.evaluate(&el_list, &eval_context);
    let prev_var = eval_context.data.get_variable(PREV_TOKEN_KEY);
    assert_eq!(prev_var, Value::default().with_int(1));
  }

  #[test]
  fn test_execution_on_first_level() {
    let eval = TokenEval::new(Rc::new(DecimalOp::default()));
    let eval_context = EvalContext::dumb();
    eval_context.data.set_eval_level(3);
    let el_list = ElementList::with_decimal(dec!(1), "1");
    let _ = eval.evaluate(&el_list, &eval_context);
    let prev_var = eval_context.data.get_variable(PREV_TOKEN_KEY);
    assert_eq!(prev_var, Value::default());
  }

  #[test]
  fn test_sum() {
    let eval = TokenEval::new(Rc::new(DecimalOp::default()));
    let eval_context = EvalContext::dumb();
    eval_context
      .data
      .set_variable(SUM_TOKEN_KEY, &Value::default().with_int(2));
    let el_list = ElementList::with_decimal(dec!(1), "1");
    let _ = eval.evaluate(&el_list, &eval_context);
    let sum_var = eval_context.data.get_variable(SUM_TOKEN_KEY);
    assert_eq!(sum_var, Value::default().with_int(3));
  }

  #[test]
  fn test_no_sum_for_empty_line() {
    let eval = TokenEval::new(Rc::new(DecimalOp::default()));
    let eval_context = EvalContext::dumb();
    eval_context
      .data
      .set_variable(SUM_TOKEN_KEY, &Value::default());
    let el_list = ElementList::with_decimal(dec!(1), "1");
    let _ = eval.evaluate(&el_list, &eval_context);
    let sum_var = eval_context.data.get_variable(SUM_TOKEN_KEY);
    assert_eq!(sum_var, Value::default().with_int(1));
  }

  #[test]
  fn test_avg() {
    let eval = TokenEval::new(Rc::new(DecimalOp::default()));
    let eval_context = EvalContext::dumb();
    eval_context
      .data
      .set_variable(CNT_TOKEN_KEY, &Value::default().with_int(1));
    eval_context
      .data
      .set_variable(SUM_TOKEN_KEY, &Value::default().with_int(2));
    let el_list = ElementList::with_decimal(dec!(4), "4");
    let _ = eval.evaluate(&el_list, &eval_context);
    let sum_var = eval_context.data.get_variable(SUM_TOKEN_KEY);
    assert_eq!(sum_var, Value::default().with_int(6));
    let avg_var = eval_context.data.get_variable(AVG_TOKEN_KEY);
    assert_eq!(avg_var, Value::default().with_int(3));
  }
}
