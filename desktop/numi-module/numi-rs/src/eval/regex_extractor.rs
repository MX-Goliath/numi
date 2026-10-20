use crate::*;
use regex::Regex;
use std::rc::Rc;

pub struct RegexExtractor<'a> {
  range_detector: Rc<StrRanger>,
  regex: Regex,
  conv_fn: Box<dyn Fn(&str) -> RegexExtractorFnReturn + 'a>,
}

type RegexExtractorFnReturn = Option<(Value, AttrKind)>;

impl<'a> RegexExtractor<'a> {
  pub fn new<F>(range_detector: Rc<StrRanger>, regex: Regex, closure: F) -> Self
  where
    F: Fn(&str) -> Option<(Value, AttrKind)> + 'a,
  {
    RegexExtractor {
      range_detector,
      regex,
      conv_fn: Box::new(closure),
    }
  }
}

impl<'a> Evaluator for RegexExtractor<'a> {
  fn evaluate<'el>(
    &self,
    elements: &ElementList<'el>,
    _context: &EvalContext,
  ) -> Option<ElementList<'el>> {
    if let Some(first_el) = elements.first() {
      let text = first_el.attr_text.str_in_range();
      let substrs = self.range_detector.substrs_for_regex(text, &self.regex);
      let res_elements: Vec<Element> = substrs
        .into_iter()
        .filter_map(|e| {
          let e_str = e.str_in_range();
          if let Some((value, attr)) = (self.conv_fn)(e_str) {
            let attr_text = AttrText::with_substr_of_kind(e.clone(), attr);
            let element = Element::new(value, attr_text);
            return Some(element);
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
mod regex_extractor_tests {

  use crate::element_list_builder::ElementListBuilder;

  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_no_extractions() {
    let regex = Regex::new(r"[0-9].").unwrap();
    let range_detector = Rc::new(StrRanger::new());
    let extractor = RegexExtractor::new(range_detector, regex, |_str: &str| None);
    let result = extractor.evaluate(&ElementList::with_str("abc"), &EvalContext::dumb());
    assert_eq!(result.unwrap(), ElementList::new());
  }

  #[test]
  fn test_extraction() {
    let regex = Regex::new(r"[0-9]+").unwrap();
    let range_detector = Rc::new(StrRanger::new());
    let extractor = RegexExtractor::new(range_detector, regex, |str: &str| {
      if let Ok(num) = str.parse::<isize>() {
        return Some((Value::empty().with_int(num), AttrKind::Number));
      }
      None
    });
    let result = extractor.evaluate(&ElementList::with_str("a12b"), &EvalContext::dumb());
    let elb = ElementListBuilder::default()
      .add_text("a")
      .add_int("12", 12)
      .add_text("b");
    assert_eq!(result.unwrap(), elb.element_list_in_range(1..2));
  }
}
