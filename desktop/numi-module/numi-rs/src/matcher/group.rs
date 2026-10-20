use super::{Match, Matcher, MatcherBox};

pub struct GroupMatcher {
  group_id: isize,
  matcher: MatcherBox,
}

impl GroupMatcher {
  pub fn new(group_id: isize, matcher: MatcherBox) -> Self {
    Self { group_id, matcher }
  }
}

impl Matcher for GroupMatcher {
  fn matches(&self, element_list: &crate::element_list::ElementList, from: isize) -> Vec<Match> {
    let mut result = Vec::new();
    let matches = self.matcher.matches(element_list, from);
    for m in matches {
      let mut groups = m.groups.clone();
      let arr = groups.entry(self.group_id).or_default();
      arr.push(m.range.clone());
      let result_m = Match::new(m.range, groups);
      result.push(result_m);
    }
    result
  }
}

#[cfg(test)]
mod group_matcher_test {
  use crate::element_list_builder::ElementListBuilder;
  use crate::op::OpMatcher;
  use crate::op::OpType;
  use std::collections::HashMap;

  use super::*;

  #[test]
  fn test() {
    let elb = ElementListBuilder::default().add_op("+", "+");
    let matcher = GroupMatcher::new(2, Box::new(OpMatcher::with_op(OpType::with_some("+"))));
    let matches = matcher.matches(&elb.element_list(), 0);
    let mut expected_groups = HashMap::new();
    expected_groups.insert(2, vec![0..1]);
    let expected_match = Match::new(0..1, expected_groups);
    assert_eq!(matches, vec![expected_match]);
  }
}
