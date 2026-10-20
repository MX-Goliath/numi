use super::{DateOp, DateOpMatcher};
use crate::matcher::Matcher;
use crate::{op::BinaryOpEvaluator, ElementList, EvalContext, Evaluator};
use std::rc::Rc;

pub struct DateOpEval {
  eval: BinaryOpEvaluator<DateOp>,
}

impl DateOpEval {
  pub fn new(binary_op: Rc<DateOp>) -> Self {
    Self {
      eval: BinaryOpEvaluator {
        matcher: DateOpMatcher::with_ops(vec!["+", "-"]).boxed(),
        binary_op,
      },
    }
  }
}

impl Evaluator for DateOpEval {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    self.eval.evaluate(elements, context)
  }
}
