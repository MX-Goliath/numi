use crate::op::OpMatcher;
use crate::op::OpType;
use crate::{
  element_list::ElementList,
  eval_context::EvalContext,
  matcher::{DecimalMatcher, Match, Matcher, MatcherBox, ReprMatcher, SequenceMatcher},
};

use super::Evaluator;

#[derive(Default)]
pub struct ReprConverterEval {
  matcher: ReprConversionMatcher,
}

impl Evaluator for ReprConverterEval {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    _context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    let matches = self.matcher.matches(elements, 0);
    let m = matches.first()?;

    let mut value = elements.get(m.range.start).unwrap().value.clone();
    let to_repr_value = &elements.get(m.range.start + 2).unwrap().value;
    value.add_repr(to_repr_value.repr().unwrap());

    let mut result = elements.clone();
    result.replace_elements_with_value(m.range.clone(), value);
    result.some()
  }
}

pub struct ReprConversionMatcher {
  matcher: MatcherBox,
}

impl Default for ReprConversionMatcher {
  fn default() -> Self {
    Self {
      matcher: SequenceMatcher::with_matchers(vec![
        DecimalMatcher::new().boxed(),
        OpMatcher::with_op(OpType::Some("in".to_string())).boxed(),
        ReprMatcher::default().boxed(),
      ])
      .boxed(),
    }
  }
}

impl Matcher for ReprConversionMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    self.matcher.matches(element_list, from)
  }
}

#[cfg(test)]
mod repr_eval_test {

  use super::*;
  use crate::{element::Element, value::repr, Value};

  #[test]
  fn test() {
    let eval = ReprConverterEval::default();
    let list_b = ElementList::builder()
      .add_int("1", 1)
      .add_op(" in ", "in")
      .add_repr("sci", repr::SCIENTIFIC);
    let result = eval
      .evaluate(&list_b.element_list(), &EvalContext::dumb())
      .unwrap();
    let expected_value = Value::default().with_int(1).with_repr(repr::SCIENTIFIC);
    assert_eq!(
      result,
      Element::new(expected_value, list_b.element_list().attr_text().unwrap()).into_list()
    );
  }
}

#[cfg(test)]
mod matcher_test {

  use super::*;
  use crate::{element_list_builder::ElementListBuilder, matcher::Match, value::repr};

  #[test]
  fn test() {
    let matcher = ReprConversionMatcher::default();
    let elb = ElementListBuilder::default()
      .add_int("1", 1)
      .add_op("in", "in")
      .add_repr("m", repr::SCIENTIFIC);
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..3)]);
  }
}
