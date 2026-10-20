use crate::eval::*;
use crate::eval_context::DataContext;
use crate::matcher::*;
use crate::*;
use std::rc::Rc;

/// This struct implements the `Evaluator` trait to evaluate binary operations
/// on elements in an `ElementList`. It uses a `Matcher` to identify binary operations
/// and a `BinaryOp` implementation to perform the actual operation.
pub struct BinaryOpEvaluator<T>
where
  T: BinaryOp,
{
  pub matcher: MatcherBox,
  pub binary_op: Rc<T>,
}

/// Trait for implementing binary operations on values.
///
/// This trait defines a method for evaluating binary operations between two `Value` operands.
/// Implementors should provide logic for specific binary operations like addition, subtraction, etc.
pub trait BinaryOp {
  fn evaluate_op(
    &self,
    op_id: &str,
    left: &Value,
    right: &Value,
    context: &DataContext,
  ) -> Option<Value>;
}

impl<T> Evaluator for BinaryOpEvaluator<T>
where
  T: BinaryOp,
{
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    let matches = self.matcher.matches(elements, 0);
    let m = matches.first()?;
    let left_el = elements.get(m.range.start)?;
    let right_el = elements.get(m.range.end - 1)?;
    let op_el = elements.get(m.range.start + 1).unwrap();
    let op_id = op_el.value.op()?;
    let result_value =
      self
        .binary_op
        .evaluate_op(op_id, &left_el.value, &right_el.value, &context.data)?;
    let mut result = elements.clone();
    result.replace_elements_with_value(m.range.clone(), result_value);
    result.some()
  }
}
