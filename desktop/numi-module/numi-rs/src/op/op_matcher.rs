use super::OpType;
use crate::matcher::FinderMatcher;
use crate::matcher::OrMatcher;
use crate::ElementList;
use crate::Match;
use crate::Matcher;

pub struct OpsMatcher {
  matcher: Box<dyn Matcher>,
}

impl OpsMatcher {
  pub fn all_operations_finder() -> Self {
    Self::operations_finder(vec![OpType::Any])
  }

  pub fn operations_finder(operations: Vec<OpType>) -> Self {
    let ops_matchers: Vec<Box<dyn Matcher>> = operations
      .into_iter()
      .map(|op| Box::new(OpMatcher::with_op(op)) as Box<dyn Matcher>)
      .collect();

    let or_matcher = OrMatcher::with_matchers(ops_matchers);
    let finder_matcher = FinderMatcher::with_matcher(Box::new(or_matcher));

    OpsMatcher {
      matcher: Box::new(finder_matcher),
    }
  }
}

impl Matcher for OpsMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    self.matcher.matches(element_list, from)
  }
}

pub struct OpMatcher {
  op: OpType,
}

impl OpMatcher {
  pub fn with_op(op: OpType) -> OpMatcher {
    OpMatcher { op }
  }
}

impl Matcher for OpMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    if let Some(el) = element_list.get(from) {
      if let Some(el_op) = el.value.op() {
        let is_matches = match &self.op {
          OpType::Any => true,
          OpType::Some(required_op) => required_op == el_op,
        };
        if is_matches {
          return vec![Match::with_element_at(from)];
        }
      }
    }
    Vec::new()
  }
}

#[cfg(test)]
mod test {

  use super::*;
  use crate::{element_list_builder::ElementListBuilder, *};

  #[test]
  fn test_any_operations() {
    let elb = ElementListBuilder::default()
      .add_text("a")
      .add_op("+", "+")
      .add_text("b")
      .add_op("-", "-")
      .add_text("c");
    let matcher = OpsMatcher::all_operations_finder();
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(
      matches,
      vec![Match::with_element_at(1), Match::with_element_at(3)]
    );
  }

  #[test]
  fn test_some_ops() {
    let elb = ElementListBuilder::default()
      .add_text("a")
      .add_op("+", "+")
      .add_text("b")
      .add_op("-", "-")
      .add_text("c");
    let matcher = OpsMatcher::operations_finder(vec![OpType::with_some("-")]);
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_element_at(3)]);
  }
}
