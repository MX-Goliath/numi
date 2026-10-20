use crate::{
  matcher::{DecimalMatcher, FinderMatcher, SequenceMatcher, UnitMatcher},
  *,
};

use self::matcher::{GroupMatcher, OrMatcher};

pub struct DecimalUnitEvaluator {
  matcher: DecimalUnitMatcher,
}

impl DecimalUnitEvaluator {
  pub fn new() -> Self {
    Self {
      matcher: DecimalUnitMatcher::new(),
    }
  }
}

impl Evaluator for DecimalUnitEvaluator {
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

    for m in matches {
      let dec_index = m.groups.get(&1).unwrap()[0].start;
      let unit_index = m.groups.get(&2).unwrap()[0].start;
      let mut value = result.get(dec_index).unwrap().value.clone();
      let unit_value = &result.get(unit_index).unwrap().value;
      value.add_unit(unit_value.unit().unwrap());
      result.replace_elements_with_value(m.range, value)
    }

    result.some()
  }
}

#[cfg(test)]
mod decimal_unit_eval_test {
  use super::*;

  #[test]
  fn test() {
    let eval = DecimalUnitEvaluator::new();
    let list_b = ElementList::builder().add_int("1", 1).add_unit("m", "m");
    let result = eval
      .evaluate(&list_b.element_list(), &EvalContext::dumb())
      .unwrap();
    let mut expected_value = Value::default().with_int(1);
    expected_value.add_unit("m");
    assert_eq!(
      result,
      Element::new(expected_value, list_b.element_list().attr_text().unwrap()).into_list()
    );
  }
}

pub struct DecimalUnitMatcher {
  matcher: FinderMatcher,
}

impl DecimalUnitMatcher {
  fn new() -> Self {
    Self {
      matcher: FinderMatcher::with_matcher(
        OrMatcher::with_matchers(vec![
          SequenceMatcher::with_matchers(vec![
            GroupMatcher::new(1, DecimalMatcher::new().boxed()).boxed(),
            GroupMatcher::new(2, UnitMatcher::default().boxed()).boxed(),
          ])
          .boxed(),
          SequenceMatcher::with_matchers(vec![
            GroupMatcher::new(2, UnitMatcher::default().boxed()).boxed(),
            GroupMatcher::new(1, DecimalMatcher::new().boxed()).boxed(),
          ])
          .boxed(),
        ])
        .boxed(),
      ),
    }
  }
}

impl Matcher for DecimalUnitMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    self.matcher.matches(element_list, from)
  }
}

#[cfg(test)]
mod decimal_unit_matcher_test {
  use crate::{element_list_builder::ElementListBuilder, matcher::MatchGroups};

  use super::*;

  #[test]
  fn test_dec_unit() {
    let matcher = DecimalUnitMatcher::new();
    let elb = ElementListBuilder::default()
      .add_int("1", 1)
      .add_unit("m", "m");
    let matches = matcher.matches(&elb.element_list(), 0);
    let mut expected_groups = MatchGroups::new();
    expected_groups.insert(1, vec![0..1]);
    expected_groups.insert(2, vec![1..2]);
    assert_eq!(matches, vec![Match::new(0..2, expected_groups)]);
  }

  #[test]
  fn test_unit_dec() {
    let matcher = DecimalUnitMatcher::new();
    let elb = ElementListBuilder::default()
      .add_unit("m", "m")
      .add_int("1", 1);
    let matches = matcher.matches(&elb.element_list(), 0);
    let mut expected_groups = MatchGroups::new();
    expected_groups.insert(2, vec![0..1]);
    expected_groups.insert(1, vec![1..2]);
    assert_eq!(matches, vec![Match::new(0..2, expected_groups)]);
  }
}
