use crate::ElementList;
use crate::Match;
use crate::Matcher;

use super::VecMatch;

pub struct AndMatcher {
  matchers: Vec<Box<dyn Matcher>>,
}

impl AndMatcher {
  pub fn with_matchers(matchers: Vec<Box<dyn Matcher>>) -> Self {
    Self { matchers }
  }
}

impl Matcher for AndMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    let mut result = Vec::new();
    for matcher in &self.matchers {
      let matches = matcher.matches(element_list, from);
      if matches.is_empty() {
        return Vec::new();
      }
      result.append_unique_matches(matches);
    }
    result
  }
}

#[cfg(test)]
mod and_matcher_test {

  use super::*;
  use crate::element_list_builder::*;
  use crate::matcher::*;

  #[test]
  fn test() {
    let elb = ElementListBuilder::default().add_int_unit("4cm", 4, "cm");
    let matcher = AndMatcher::with_matchers(vec![
      Box::<UnitMatcher>::default(),
      Box::new(DecimalMatcher::new()),
    ]);
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_element_at(0)]);
  }
}
