use crate::{
  attr_text::AttrKind, date::DateTime, eval::RegexExtractor, str_ranger::StrRanger, Evaluator,
  Value,
};
use regex::Regex;
use std::rc::Rc;

pub struct DateExtractor<'a> {
  regex_extractor: RegexExtractor<'a>,
}

pub const DEFAULT_DATE_PRIORITY: isize = 100;

impl DateExtractor<'_> {
  pub fn new(range_detector: Rc<StrRanger>) -> Self {
    let regex_extractor = RegexExtractor::new(
      range_detector,
      Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap(),
      |str: &str| {
        if let Some(datetime) = DateTime::with_str_date(str) {
          let value = Value::default()
            .with_datetime(datetime)
            .with_priority(DEFAULT_DATE_PRIORITY);
          return Some((value, AttrKind::Number));
        }
        None
      },
    );
    Self { regex_extractor }
  }
}

impl Evaluator for DateExtractor<'_> {
  fn evaluate<'a>(
    &self,
    elements: &crate::element_list::ElementList<'a>,
    context: &crate::eval_context::EvalContext,
  ) -> Option<crate::element_list::ElementList<'a>> {
    self.regex_extractor.evaluate(elements, context)
  }
}

#[cfg(test)]
mod date_extractor_tests {

  use super::*;
  use crate::{
    date::DateTime, element_list::ElementList, element_list_builder::ElementListBuilder,
    eval_context::EvalContext,
  };

  #[test]
  fn test_no_date() {
    let extractor = DateExtractor::new(Rc::new(StrRanger::new()));
    let result = extractor.evaluate(&ElementList::with_str("yui"), &EvalContext::dumb());
    assert_eq!(result.unwrap(), ElementList::new());
  }

  #[test]
  fn test_date() {
    let extractor = DateExtractor::new(Rc::new(StrRanger::new()));
    let elb = ElementListBuilder::default()
      .add_date(
        "2021-01-01",
        DateTime::with_str_datetime("2021-01-01 00:00:00").unwrap(),
      )
      .with_priority(DEFAULT_DATE_PRIORITY);
    assert_eq!(
      extractor
        .evaluate(&ElementList::with_str("2021-01-01"), &EvalContext::dumb())
        .unwrap(),
      elb.element_list()
    );
  }
}
