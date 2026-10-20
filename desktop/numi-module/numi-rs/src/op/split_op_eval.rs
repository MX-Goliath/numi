use super::OpRegistry;
use crate::eval::SplitEvaluator;
use crate::op::OpType;
use crate::op::SplitOpMatcher;
use crate::Element;
use crate::ElementList;
use crate::EvalContext;
use crate::Evaluator;
use std::{cell::RefCell, rc::Rc};

/// An evaluator that splits and evaluates expressions based on binary operations.
/// This struct implements the `Evaluator` trait to split complex expressions
/// into simpler sub-expressions based on the priority of binary operations.
/// It uses a weight-based system to determine the order in which operations
/// should be evaluated, with higher weights being evaluated first.
pub struct SplitOpEvaluator {
  decimal_op: Rc<RefCell<OpRegistry>>,
}

impl SplitOpEvaluator {
  pub fn new(decimal_op: Rc<RefCell<OpRegistry>>) -> Self {
    Self { decimal_op }
  }
}

impl Evaluator for SplitOpEvaluator {
  fn evaluate<'b>(
    &self,
    elements: &ElementList<'b>,
    context: &EvalContext,
  ) -> Option<ElementList<'b>> {
    let ops = self.decimal_op.borrow().binary_op_map();
    let just_ops = ops.keys().cloned().collect();
    let matcher = SplitOpMatcher::with_operations(just_ops);
    let evaluator = SplitEvaluator::new(Box::new(matcher), move |el: &Element| {
      if let Some(op) = el.value.op() {
        if let Some(weight) = ops.get(&OpType::Some(op.to_string())) {
          return *weight;
        }
      }
      0
    });
    evaluator.evaluate(elements, context)
  }
}
