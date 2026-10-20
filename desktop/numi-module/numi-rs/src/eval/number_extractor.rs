use crate::str_ext::StrExt;
use crate::*;
use regex::Regex;
use rust_decimal::prelude::*;
use std::rc::Rc;

pub struct NumberExtractor {
  range_detector: Rc<StrRanger>,
  number_regex: Regex,
}

impl NumberExtractor {
  pub fn new(range_detector: Rc<StrRanger>) -> NumberExtractor {
    NumberExtractor {
      range_detector,
      number_regex: Regex::new(r"[.\d](?:\s*[.,\d])*").unwrap(),
    }
  }
}

impl Evaluator for NumberExtractor {
  fn evaluate<'el>(
    &self,
    elements: &ElementList<'el>,
    _context: &EvalContext,
  ) -> Option<ElementList<'el>> {
    if let Some(first_el) = elements.first() {
      let text = first_el.attr_text.str_in_range();
      let substrs = self
        .range_detector
        .substrs_for_regex(text, &self.number_regex);
      let res_elements: Vec<Element> = substrs
        .into_iter()
        .filter_map(|e| {
          let e_str = e
            .str_in_range()
            .string_without_whitespaces()
            .replace(',', "");
          if let Ok(num) = Decimal::from_str(&e_str) {
            return Option::Some(Element::with_decimal_in_substr(num, e.clone()));
          }
          Option::None
        })
        .collect();
      ElementList::with_elements(res_elements).some()
    } else {
      None
    }
  }
}

#[cfg(test)]
mod number_extractor_tests {

  use super::*;
  use pretty_assertions::assert_eq;
  use rust_decimal_macros::dec;

  #[test]
  fn test_no_numbers() {
    let result =
      build_number_extractor().evaluate(&ElementList::with_str("abc"), &EvalContext::dumb());
    assert_eq!(result.unwrap(), ElementList::new());
  }

  #[test]
  fn test_number() {
    assert_eq!(
      build_number_extractor()
        .evaluate(&ElementList::with_str("123"), &EvalContext::dumb())
        .unwrap(),
      ElementList::with_decimal(dec!(123), "123")
    );
  }

  #[test]
  fn test_decimal_number_with_dot() {
    assert_eq!(
      build_number_extractor()
        .evaluate(&ElementList::with_str("1.2"), &EvalContext::dumb())
        .unwrap(),
      ElementList::with_decimal(dec!(1.2), "1.2")
    );
  }

  #[test]
  fn test_with_space() {
    assert_eq!(
      build_number_extractor()
        .evaluate(&ElementList::with_str("1 2"), &EvalContext::dumb())
        .unwrap(),
      ElementList::with_decimal(dec!(12), "1 2")
    );
  }

  #[test]
  fn test_with_comma() {
    assert_eq!(
      build_number_extractor()
        .evaluate(&ElementList::with_str("1,000"), &EvalContext::dumb())
        .unwrap(),
      ElementList::with_decimal(dec!(1000), "1,000")
    );
  }
  // Helpers

  fn build_number_extractor() -> NumberExtractor {
    let range_detector = Rc::new(StrRanger::new());
    NumberExtractor::new(range_detector)
  }
}
