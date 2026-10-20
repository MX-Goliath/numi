use crate::ElementList;
use crate::Match;
use crate::Matcher;

pub struct OrMatcher {
  matchers: Vec<Box<dyn Matcher>>,
}

impl OrMatcher {
  pub fn with_matchers(matchers: Vec<Box<dyn Matcher>>) -> Self {
    Self { matchers }
  }
}

impl Matcher for OrMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    let mut result = Vec::new();
    for matcher in &self.matchers {
      let mut matches = matcher.matches(element_list, from);
      if !matches.is_empty() {
        result.append(&mut matches);
      }
    }
    result
  }
}

#[cfg(test)]
mod or_matcher_test {

  use super::*;
  use crate::element_list_builder::*;
  use crate::matcher::*;
  use crate::op::OpMatcher;
  use crate::op::OpType;

  #[test]
  fn test() {
    let elb = ElementListBuilder::default().add_op("+", "+");
    let matcher = OrMatcher::with_matchers(vec![
      Box::new(OpMatcher::with_op(OpType::with_some("-"))),
      Box::new(OpMatcher::with_op(OpType::with_some("+"))),
    ]);
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_element_at(0)]);
  }
}
