use super::{regex_extractor::RegexExtractor, Evaluator};
use crate::{attr_text::AttrKind, str_ranger::StrRanger, value::repr, Value};
use regex::Regex;
use std::rc::Rc;

pub struct OctExtractor<'a> {
  regex_extractor: RegexExtractor<'a>,
}

impl OctExtractor<'_> {
  pub fn new(range_detector: Rc<StrRanger>) -> Self {
    let regex_extractor = RegexExtractor::new(
      range_detector,
      Regex::new(r"0[oO]([0-7]+)").unwrap(),
      |str: &str| {
        let digits_str = &str[2..];
        if let Ok(num) = isize::from_str_radix(digits_str, 8) {
          let value = Value::default()
            .with_int(num)
            .with_repr(repr::OCTAL)
            .with_priority(20); // priority should be higher or may conflict with other values
          return Some((value, AttrKind::Number));
        }
        None
      },
    );
    Self { regex_extractor }
  }
}

impl Evaluator for OctExtractor<'_> {
  fn evaluate<'a>(
    &self,
    elements: &crate::element_list::ElementList<'a>,
    context: &crate::eval_context::EvalContext,
  ) -> Option<crate::element_list::ElementList<'a>> {
    self.regex_extractor.evaluate(elements, context)
  }
}

#[cfg(test)]
mod oct_extractor_tests {

  use super::*;
  use crate::{
    element_list::ElementList, element_list_builder::ElementListBuilder, eval_context::EvalContext,
    value::repr,
  };
  use rust_decimal_macros::dec;

  #[test]
  fn test_no_numbers() {
    let extractor = OctExtractor::new(Rc::new(StrRanger::new()));
    let result = extractor.evaluate(&ElementList::with_str("yui"), &EvalContext::dumb());
    assert_eq!(result.unwrap(), ElementList::new());
  }

  #[test]
  fn test_number() {
    let extractor = OctExtractor::new(Rc::new(StrRanger::new()));
    let elb = ElementListBuilder::default()
      .add_decimal_with_repr("0o5", dec!(5), repr::OCTAL)
      .with_priority(20);
    assert_eq!(
      extractor
        .evaluate(&ElementList::with_str("0o5"), &EvalContext::dumb())
        .unwrap(),
      elb.element_list()
    );
  }
}
