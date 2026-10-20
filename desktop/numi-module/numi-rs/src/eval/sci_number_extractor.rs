use crate::{value::repr, *};
use regex::Regex;
use rust_decimal::prelude::*;
use std::rc::Rc;

use super::regex_extractor::RegexExtractor;

pub struct SciNumberExtractor<'a> {
  regex_extractor: RegexExtractor<'a>,
}

impl SciNumberExtractor<'_> {
  pub fn new(range_detector: Rc<StrRanger>) -> Self {
    let regex_extractor = RegexExtractor::new(
      range_detector,
      Regex::new(r"(?x)\.? \d+ [\s\d\._,]* [Ee] [+-]? \d+").unwrap(),
      |str: &str| {
        if let Ok(num) = Decimal::from_scientific(str) {
          let value = Value::default()
            .with_number(num)
            .with_repr(repr::SCIENTIFIC)
            .with_priority(1);
          return Some((value, AttrKind::Number));
        }
        None
      },
    ); // (?x) to ignore spaces
    SciNumberExtractor { regex_extractor }
  }
}

impl Evaluator for SciNumberExtractor<'_> {
  fn evaluate<'el>(
    &self,
    elements: &ElementList<'el>,
    context: &EvalContext,
  ) -> Option<ElementList<'el>> {
    self.regex_extractor.evaluate(elements, context)
  }
}

#[cfg(test)]
mod sci_number_extractor_tests {

  use super::*;
  use crate::{element_list_builder::ElementListBuilder, value::repr};
  use pretty_assertions::assert_eq;
  use rust_decimal_macros::dec;

  #[test]
  fn test_no_numbers() {
    let extractor = SciNumberExtractor::new(Rc::new(StrRanger::new()));
    let result = extractor.evaluate(&ElementList::with_str("abc"), &EvalContext::dumb());
    assert_eq!(result.unwrap(), ElementList::new());
  }

  #[test]
  fn test_number() {
    let extractor = SciNumberExtractor::new(Rc::new(StrRanger::new()));
    let elb = ElementListBuilder::default()
      .add_decimal_with_repr("5e2", dec!(500), repr::SCIENTIFIC)
      .with_priority(1);
    assert_eq!(
      extractor
        .evaluate(&ElementList::with_str("5e2"), &EvalContext::dumb())
        .unwrap(),
      elb.element_list()
    );
  }
}
