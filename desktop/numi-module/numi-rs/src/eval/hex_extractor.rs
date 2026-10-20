use super::{regex_extractor::RegexExtractor, Evaluator};
use crate::{attr_text::AttrKind, str_ranger::StrRanger, value::repr, Value};
use regex::Regex;
use std::rc::Rc;

pub struct HexExtractor<'a> {
  regex_extractor: RegexExtractor<'a>,
}

impl HexExtractor<'_> {
  pub fn new(range_detector: Rc<StrRanger>) -> Self {
    let regex_extractor = RegexExtractor::new(
      range_detector,
      Regex::new(r"0[xX]([0-9a-fA-F]+)").unwrap(),
      |str: &str| {
        let digits_str = &str[2..];
        if let Ok(num) = isize::from_str_radix(digits_str, 16) {
          let value = Value::default().with_int(num).with_repr(repr::HEX);
          return Some((value, AttrKind::Number));
        }
        None
      },
    );
    Self { regex_extractor }
  }
}

impl Evaluator for HexExtractor<'_> {
  fn evaluate<'a>(
    &self,
    elements: &crate::element_list::ElementList<'a>,
    context: &crate::eval_context::EvalContext,
  ) -> Option<crate::element_list::ElementList<'a>> {
    self.regex_extractor.evaluate(elements, context)
  }
}

#[cfg(test)]
mod hex_extractor_tests {

  use super::*;
  use crate::{
    element_list::ElementList, element_list_builder::ElementListBuilder, eval_context::EvalContext,
    value::repr,
  };
  use rust_decimal_macros::dec;

  #[test]
  fn test_no_numbers() {
    let extractor = HexExtractor::new(Rc::new(StrRanger::new()));
    let result = extractor.evaluate(&ElementList::with_str("yui"), &EvalContext::dumb());
    assert_eq!(result.unwrap(), ElementList::new());
  }

  #[test]
  fn test_number() {
    let extractor = HexExtractor::new(Rc::new(StrRanger::new()));
    let elb = ElementListBuilder::default().add_decimal_with_repr("0xff", dec!(255), repr::HEX);
    assert_eq!(
      extractor
        .evaluate(&ElementList::with_str("0xff"), &EvalContext::dumb())
        .unwrap(),
      elb.element_list()
    );
  }

  #[test]
  fn test_number_caps() {
    let extractor = HexExtractor::new(Rc::new(StrRanger::new()));
    let elb = ElementListBuilder::default().add_decimal_with_repr("0XFF", dec!(255), repr::HEX);
    assert_eq!(
      extractor
        .evaluate(&ElementList::with_str("0XFF"), &EvalContext::dumb())
        .unwrap(),
      elb.element_list()
    );
  }
}
