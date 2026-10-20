use crate::eval::*;
use crate::op::BinaryOpEvaluator;
use crate::op::DecimalOp;
use crate::*;
use op::DecimalOpMatcher;
use std::rc::Rc;

pub struct DecimalOpEvaluator {
  evaluator: BinaryOpEvaluator<DecimalOp>,
}

impl DecimalOpEvaluator {
  pub fn new(binary_op: Rc<DecimalOp>) -> Self {
    Self {
      evaluator: BinaryOpEvaluator {
        matcher: DecimalOpMatcher::with_ops(vec![
          "+", "-", "*", "/", "exp", "mod", "and", "or", "xor", "rshift", "lshift",
        ])
        .boxed(),
        binary_op,
      },
    }
  }
}

impl Evaluator for DecimalOpEvaluator {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    self.evaluator.evaluate(elements, context)
  }
}

#[cfg(test)]
mod decimal_evaluator_test {

  use super::*;
  use crate::{element_list_builder::ElementListBuilder, unit_converter::UnitConverter};
  use pretty_assertions::assert_eq;
  use std::cell::RefCell;

  #[test]
  fn test_should_return_none_if_cant_evaluate() {
    let conv = Rc::new(RefCell::new(UnitConverter::default()));
    let binary_op = Rc::new(DecimalOp::new(conv));
    let evaluator = DecimalOpEvaluator::new(binary_op);
    let elb = ElementListBuilder::default().add_text("a");
    let result = evaluator.evaluate(&elb.element_list(), &EvalContext::dumb());
    assert_eq!(result, None);
  }

  #[test]
  fn test_sum() {
    let conv = Rc::new(RefCell::new(UnitConverter::default()));
    let binary_op = Rc::new(DecimalOp::new(conv));
    let evaluator = DecimalOpEvaluator::new(binary_op);
    let elb = ElementListBuilder::default()
      .add_int("1", 1)
      .add_op("+", "+")
      .add_int("2", 2);
    let result = evaluator.evaluate(&elb.element_list(), &EvalContext::dumb());
    let expected_attr_str = elb.element_list().attr_text().unwrap();
    let expected_value = Value::default().with_int(3);
    let expected_list = ElementList::with_element(Element::new(expected_value, expected_attr_str));
    assert_eq!(result.unwrap(), expected_list);
  }
}
