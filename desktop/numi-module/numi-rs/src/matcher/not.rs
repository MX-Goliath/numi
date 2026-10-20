use crate::ElementList;
use crate::Match;
use crate::Matcher;

pub struct NotMatcher {
  matcher: Box<dyn Matcher>,
}

impl NotMatcher {
  pub fn with_matcher(matcher: Box<dyn Matcher>) -> Self {
    Self { matcher }
  }
}

impl Matcher for NotMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    if from < element_list.len() {
      let matches = self.matcher.matches(element_list, from);
      if matches.is_empty() {
        return vec![Match::with_element_at(from)];
      }
    }
    Vec::new()
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
  fn test_match() {
    let elb = ElementListBuilder::default().add_op("+", "+");
    let matcher = NotMatcher::with_matcher(Box::new(OpMatcher::with_op(OpType::with_some("-"))));
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_element_at(0)]);
  }

  #[test]
  fn test_no_match() {
    let elb = ElementListBuilder::default().add_op("+", "+");
    let matcher = NotMatcher::with_matcher(Box::new(OpMatcher::with_op(OpType::with_some("+"))));
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![]);
  }
}
