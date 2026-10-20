use crate::{op::OpRegistry, *};
use std::{cell::RefCell, rc::Rc};

// Some operation tokens consiste of multiple words
// and may overlap with units, with priority set to 10
pub const DEFAULT_OP_PRIORITY: isize = 20;

pub struct OpExtractor {
  decimal_op: Rc<RefCell<OpRegistry>>,
}

impl OpExtractor {
  pub fn new(decimal_op: Rc<RefCell<OpRegistry>>) -> OpExtractor {
    OpExtractor { decimal_op }
  }
}

impl Evaluator for OpExtractor {
  fn evaluate<'el>(
    &self,
    elements: &ElementList<'el>,
    _context: &EvalContext,
  ) -> Option<ElementList<'el>> {
    if let Some(first_el) = elements.first() {
      let substr = first_el.attr_text.substr.clone();
      let decimal_op = self.decimal_op.borrow();
      let matches = decimal_op.matches(substr);
      let res_elements: Vec<Element> = matches
        .into_iter()
        .map(|m| {
          return Element::new(
            Value::default()
              .with_op(m.id)
              .with_priority(DEFAULT_OP_PRIORITY),
            AttrText::with_substr_of_kind(m.substr, AttrKind::Op),
          );
        })
        .collect();
      ElementList::with_elements(res_elements).some()
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::op::Op;

  #[test]
  fn test() {
    let mut decimal_op = OpRegistry::default();
    decimal_op.add_op(Op::new("+"), None, vec!["+"]);
    let evaluator = OpExtractor::new(Rc::new(RefCell::new(decimal_op)));
    let str = "1+2";
    let input_el_list = ElementList::with_str(str);
    let result = evaluator.evaluate(&input_el_list, &EvalContext::dumb());

    let expected =
      ElementList::with_element(Element::with_op_in_substr("+", Substr::new(str, 1..2)));
    assert_eq!(result.unwrap(), expected);
  }

  #[test]
  fn test_div() {
    let mut decimal_op = OpRegistry::default();
    decimal_op.add_op(Op::new("/"), None, vec!["/"]);
    let evaluator = OpExtractor::new(Rc::new(RefCell::new(decimal_op)));
    let str = "1/2";
    let input_el_list = ElementList::with_str(str);
    let result = evaluator.evaluate(&input_el_list, &EvalContext::dumb());

    let expected =
      ElementList::with_element(Element::with_op_in_substr("/", Substr::new(str, 1..2)));
    assert_eq!(result.unwrap(), expected);
  }
  #[test]
  fn test_parentheses() {
    let mut decimal_op = OpRegistry::default();
    decimal_op.add_op(Op::new("("), None, vec!["("]);
    decimal_op.add_op(Op::new(")"), None, vec![")"]);
    let evaluator = OpExtractor::new(Rc::new(RefCell::new(decimal_op)));
    let str = "1(2)";
    let input_el_list = ElementList::with_str(str);
    let result = evaluator.evaluate(&input_el_list, &EvalContext::dumb());

    let expected = ElementList::with_elements(vec![
      Element::with_op_in_substr("(", Substr::new(str, 1..2)),
      Element::with_op_in_substr(")", Substr::new(str, 3..4)),
    ]);
    assert_eq!(result.unwrap(), expected);
  }
}
