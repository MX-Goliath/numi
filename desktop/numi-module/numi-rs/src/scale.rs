use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rust_decimal::MathematicalOps;
use rust_decimal_macros::dec;

use crate::{
  attr_text::{AttrKind, AttrText},
  element::Element,
  element_list::ElementList,
  eval::Evaluator,
  eval_context::EvalContext,
  matcher::{DecimalMatcher, FinderMatcher, Match, Matcher, SequenceMatcher},
  phrase_detector::{PhraseDetector, PhraseMatch},
  substr::Substr,
  StrRanger, Value,
};

#[derive()]
pub struct ScaleManager {
  scales: HashMap<String, Scale>,
  detector: PhraseDetector,
}

impl Default for ScaleManager {
  fn default() -> Self {
    Self {
      scales: HashMap::new(),
      detector: PhraseDetector::new(StrRanger::default(), false),
    }
  }
}

impl ScaleManager {
  pub fn add_scale(&mut self, id: &str, power: i64, phrases: &Vec<&str>) {
    self.scales.insert(
      id.to_string(),
      Scale {
        id: id.to_string(),
        power,
      },
    );
    self.detector.add_phrases_for(id, phrases);
  }

  pub fn matches<'a, 's, S>(&'a self, into_substr: S) -> Vec<PhraseMatch<'a, 's>>
  where
    S: Into<Substr<'s>>,
  {
    self.detector.matches(into_substr)
  }

  pub fn power(&self, value: &Value, scale_id: &str) -> Value {
    let mut result = value.clone();
    if let Some(num) = value.number() {
      if let Some(scale) = self.scales.get(scale_id) {
        let ten_powered = dec!(10).powi(scale.power);
        let result_num = num * ten_powered;
        result.add_number(result_num);
      }
    }
    result
  }
}

#[cfg(test)]
mod scale_manager_test {

  use super::*;

  #[test]
  fn test_power() {
    let mut manager = ScaleManager::default();
    manager.add_scale("k", 3, &vec![]);
    let result = manager.power(&Value::default().with_int(3), "k");
    assert_eq!(result, Value::default().with_int(3000));
  }
}

pub struct Scale {
  pub id: String,
  pub power: i64,
}

pub struct ScaleExtractor {
  scale_manager: Rc<RefCell<ScaleManager>>,
}

impl ScaleExtractor {
  pub fn new(scale_manager: Rc<RefCell<ScaleManager>>) -> Self {
    Self { scale_manager }
  }
}

impl Evaluator for ScaleExtractor {
  fn evaluate<'el>(
    &self,
    elements: &ElementList<'el>,
    _context: &EvalContext,
  ) -> Option<ElementList<'el>> {
    if let Some(first_el) = elements.first() {
      let substr = first_el.attr_text.substr.clone();
      let manager = self.scale_manager.borrow();
      let matches = manager.matches(substr);
      let res_elements: Vec<Element> = matches
        .into_iter()
        .map(|m| {
          return Element::new(
            // priority should be on par or higher than for units,
            // otherwise may be override (Millions, M)
            Value::default().with_scale(m.id).with_priority(10),
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
mod test_scale_extractor {
  use super::*;

  #[test]
  fn test_scale_extractor() {
    let mut manager = ScaleManager::default();
    manager.add_scale("k", 3, &vec!["tho"]);
    let rc_manager = Rc::new(RefCell::new(manager));
    let evaluator = ScaleExtractor::new(rc_manager);
    let str = "5 tho";
    let input_el_list = ElementList::with_str(str);
    let result = evaluator.evaluate(&input_el_list, &EvalContext::dumb());
    let el = Element::new(
      Value::default().with_scale("k").with_priority(10),
      AttrText::with_substr_of_kind(Substr::new(str, 2..5), AttrKind::Unit),
    );
    assert_eq!(result.unwrap(), ElementList::with_element(el));
  }
}

pub struct ScaleEvaluator {
  matcher: ScaleExpressionMatcher,
  manager: Rc<RefCell<ScaleManager>>,
}

impl ScaleEvaluator {
  pub fn new(manager: Rc<RefCell<ScaleManager>>) -> Self {
    Self {
      matcher: ScaleExpressionMatcher::default(),
      manager,
    }
  }
}

impl Evaluator for ScaleEvaluator {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    _context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    let mut matches = self.matcher.matches(elements, 0);
    if matches.is_empty() {
      return None;
    }
    matches.reverse();
    let mut result = elements.clone();
    let manager = self.manager.borrow();
    for m in matches {
      let value = result.get(m.range.start).unwrap().value.clone();
      let scale_id = result
        .get(m.range.start + 1)
        .unwrap()
        .value
        .scale()
        .unwrap();
      let res_value = manager.power(&value, scale_id);
      result.replace_elements_with_value(m.range, res_value);
    }

    result.some()
  }
}

#[cfg(test)]
mod scale_eval_test {
  use super::*;

  #[test]
  fn test() {
    let mut manager = ScaleManager::default();
    manager.add_scale("k", 3, &vec![]);
    let rc_manager = Rc::new(RefCell::new(manager));
    let eval = ScaleEvaluator::new(rc_manager);
    let list_b = ElementList::builder().add_int("2", 2).add_scale("k", "k");
    let result = eval
      .evaluate(&list_b.element_list(), &EvalContext::dumb())
      .unwrap();
    let expected_value = Value::default().with_int(2000);
    assert_eq!(
      result,
      Element::new(expected_value, list_b.element_list().attr_text().unwrap()).into_list()
    );
  }
}
pub struct ScaleExpressionMatcher {
  matcher: FinderMatcher,
}

impl Default for ScaleExpressionMatcher {
  fn default() -> Self {
    Self {
      matcher: FinderMatcher::with_matcher(
        SequenceMatcher::with_matchers(vec![
          DecimalMatcher::new().boxed(),
          ScaleMatcher::default().boxed(),
        ])
        .boxed(),
      ),
    }
  }
}

impl Matcher for ScaleExpressionMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    self.matcher.matches(element_list, from)
  }
}

#[derive(Default)]
pub struct ScaleMatcher {}

impl Matcher for ScaleMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    if let Some(el) = element_list.get(from) {
      if el.value.has_scale() {
        return vec![Match::with_element_at(from)];
      }
    }
    Vec::new()
  }
}

#[cfg(test)]
mod scale_expression_matcher_test {
  use super::*;
  use crate::element_list_builder::ElementListBuilder;

  #[test]
  fn test() {
    let matcher = ScaleExpressionMatcher::default();
    let elb = ElementListBuilder::default()
      .add_int("1", 1)
      .add_scale("k", "k");
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..2)]);
  }
}
