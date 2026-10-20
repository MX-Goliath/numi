use super::TzDateMatcher;
use crate::{matcher::*, Value};
use crate::{unit_converter::UnitConverter, ElementList, EvalContext, Evaluator};
use std::{cell::RefCell, rc::Rc};

pub struct TzDateEval {
  matcher: TzDateMatcher,
  unit_converter: Rc<RefCell<UnitConverter>>,
}

impl TzDateEval {
  pub fn new(unit_converter: Rc<RefCell<UnitConverter>>) -> Self {
    Self {
      matcher: TzDateMatcher::new(),
      unit_converter,
    }
  }
}

impl Evaluator for TzDateEval {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    let matches = self.matcher.matches(elements, 0);
    let m = matches.first()?;

    let value = elements.get(m.range.start + 1).unwrap().value.clone();
    let to = &elements.get(m.range.start).unwrap().value;

    // dbg!(&value);
    // dbg!(&to);

    let mut result = elements.clone();
    if let Some(result_value) = self
      .unit_converter
      .borrow()
      .convert(&value, to, &context.data)
    {
      result.replace_elements_with_value(m.range.clone(), result_value);
      return result.some();
    }
    result.replace_elements_with_value(0..elements.len(), Value::with_error());
    Some(result)
  }
}
