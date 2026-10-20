use crate::{
  attr_text::{AttrKind, AttrText},
  element::Element,
  element_list::ElementList,
  eval_context::EvalContext,
  phrase_detector::PhraseDetector,
  value::repr,
  Value,
};

use super::Evaluator;

#[derive(Default)]
pub struct ReprExtractor {
  detector: PhraseDetector,
}

impl ReprExtractor {
  pub fn add_repr(&mut self, id: &str, phrases: &Vec<&str>) {
    self.detector.add_phrases_for(id, phrases);
  }
}

impl Evaluator for ReprExtractor {
  fn evaluate<'el>(
    &self,
    elements: &ElementList<'el>,
    _context: &EvalContext,
  ) -> Option<ElementList<'el>> {
    if let Some(first_el) = elements.first() {
      let substr = first_el.attr_text.substr.clone();
      let matches = self.detector.matches(substr);
      let res_elements: Vec<Element> = matches
        .into_iter()
        .map(|m| {
          let mut repr = 0;
          if m.id == "dec" {
            repr = repr::DECIMAL;
          }
          if m.id == "hex" {
            repr = repr::HEX;
          }
          if m.id == "oct" {
            repr = repr::OCTAL;
          }
          if m.id == "bin" {
            repr = repr::BINARY;
          }
          if m.id == "sci" {
            repr = repr::SCIENTIFIC;
          }
          return Element::new(
            Value::default().with_repr(repr),
            AttrText::with_substr_of_kind(m.substr, AttrKind::Unit),
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
mod test {
  use crate::substr::Substr;

  use super::*;

  #[test]
  fn test_repr_extractor() {
    let mut extractor = ReprExtractor::default();
    extractor.add_repr("sci", &vec!["sci"]);
    let str = "sci";
    let input_el_list = ElementList::with_str(str);
    let result = extractor.evaluate(&input_el_list, &EvalContext::dumb());
    let el = Element::new(
      Value::default().with_repr(repr::SCIENTIFIC),
      AttrText::with_substr_of_kind(Substr::new(str, 0..3), AttrKind::Unit),
    );
    assert_eq!(result.unwrap(), ElementList::with_element(el));
  }
}
