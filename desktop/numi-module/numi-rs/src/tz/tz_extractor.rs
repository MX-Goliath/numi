use chrono_tz::TZ_VARIANTS;

const DEFAULT_PRIORITY: isize = 102;

use crate::{
  phrase_detector::PhraseDetector, AttrKind, AttrText, Element, ElementList, EvalContext,
  Evaluator, Value,
};

pub struct TzExtractor {
  detector: PhraseDetector,
}

impl Default for TzExtractor {
  fn default() -> Self {
    let mut pd = PhraseDetector::default();
    pd.add_phrases_for("UTC", &vec!["UTC"]);
    for tz in TZ_VARIANTS.iter() {
      pd.add_phrases_for(tz.name(), &vec![tz.name()]);
    }
    Self { detector: pd }
  }
}

impl Evaluator for TzExtractor {
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
          return Element::new(
            Value::default()
              .with_timezone(m.id)
              .with_priority(DEFAULT_PRIORITY),
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
