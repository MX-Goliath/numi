use crate::{
  attr_text::AttrKind, date::DateTime, eval::RegexExtractor, str_ranger::StrRanger, Evaluator,
  Value,
};
use regex::Regex;
use std::rc::Rc;

pub struct DatetimeExtractor<'a> {
  regex_extractor: RegexExtractor<'a>,
}

pub const DEFAULT_DATETIME_PRIORITY: isize = 101;

impl DatetimeExtractor<'_> {
  pub fn new(range_detector: Rc<StrRanger>) -> Self {
    let regex_extractor = RegexExtractor::new(
      range_detector,
      Regex::new(r"(\d{4}-\d{2}-\d{2} )?\d{2}:\d{2}(:\d{2})?").unwrap(),
      |str: &str| {
        if let Some(datetime) = DateTime::with_str_datetime(str) {
          let value = Value::default()
            .with_datetime(datetime)
            .with_priority(DEFAULT_DATETIME_PRIORITY);
          return Some((value, AttrKind::Number));
        }
        None
      },
    );
    Self { regex_extractor }
  }
}

impl Evaluator for DatetimeExtractor<'_> {
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
  fn test_no_datetime() {
    let extractor = DatetimeExtractor::new(Rc::new(StrRanger::new()));
    let result = extractor.evaluate(&ElementList::with_str("yui"), &EvalContext::dumb());
    assert_eq!(result.unwrap(), ElementList::new());
  }

  #[test]
  fn test_date_full() {
    let extractor = DatetimeExtractor::new(Rc::new(StrRanger::new()));
    let elb = ElementListBuilder::default()
      .add_date(
        "2021-01-01 10:20:30",
        DateTime::with_str_datetime("2021-01-01 10:20:30").unwrap(),
      )
      .with_priority(DEFAULT_DATETIME_PRIORITY);
    assert_eq!(
      extractor
        .evaluate(
          &ElementList::with_str("2021-01-01 10:20:30"),
          &EvalContext::dumb()
        )
        .unwrap(),
      elb.element_list()
    );
  }

  #[test]
  fn test_date_without_seconds() {
    let extractor = DatetimeExtractor::new(Rc::new(StrRanger::new()));
    let elb = ElementListBuilder::default()
      .add_date(
        "2021-01-01 10:20",
        DateTime::with_str_datetime("2021-01-01 10:20").unwrap(),
      )
      .with_priority(DEFAULT_DATETIME_PRIORITY);
    assert_eq!(
      extractor
        .evaluate(
          &ElementList::with_str("2021-01-01 10:20"),
          &EvalContext::dumb()
        )
        .unwrap(),
      elb.element_list()
    );
  }

  #[test]
  fn test_time() {
    let extractor = DatetimeExtractor::new(Rc::new(StrRanger::new()));
    let elb = ElementListBuilder::default()
      .add_date("10:20", DateTime::with_str_datetime("10:20").unwrap())
      .with_priority(DEFAULT_DATETIME_PRIORITY);
    assert_eq!(
      extractor
        .evaluate(&ElementList::with_str("10:20"), &EvalContext::dumb())
        .unwrap(),
      elb.element_list()
    );
  }
}
