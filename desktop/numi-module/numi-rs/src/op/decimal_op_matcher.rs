use crate::matcher::*;
use crate::*;
use op::{OpMatcher, OpType};

pub struct DecimalOpMatcher {
  matcher: MatcherBox,
}

impl DecimalOpMatcher {
  pub fn new(op: &str) -> Self {
    Self {
      matcher: SequenceMatcher::with_matchers(vec![
        DecimalMatcher::new().boxed(),
        OpMatcher::with_op(OpType::Some(op.to_string())).boxed(),
        DecimalMatcher::new().boxed(),
      ])
      .boxed(),
    }
  }

  pub fn with_ops(ops: Vec<&str>) -> Self {
    let mut op_matchers: Vec<Box<dyn Matcher>> = vec![];
    for op_id in ops.iter() {
      op_matchers.push(OpMatcher::with_op(OpType::Some(op_id.to_string())).boxed());
    }
    Self {
      matcher: SequenceMatcher::with_matchers(vec![
        DecimalMatcher::new().boxed(),
        OrMatcher::with_matchers(op_matchers).boxed(),
        DecimalMatcher::new().boxed(),
      ])
      .boxed(),
    }
  }
}

impl Matcher for DecimalOpMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    self.matcher.matches(element_list, from)
  }
}

#[cfg(test)]
mod decimal_op_matcher_test {

  use super::*;
  use crate::element_list_builder::ElementListBuilder;

  #[test]
  fn test() {
    let matcher = DecimalOpMatcher::new("+");
    let elb = ElementListBuilder::default()
      .add_int("1", 1)
      .add_op("+", "+")
      .add_int("2", 2);
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..3)]);
  }

  #[test]
  fn test_ops() {
    let matcher = DecimalOpMatcher::with_ops(vec!["+", "-"]);
    let elb = ElementListBuilder::default()
      .add_int("1", 1)
      .add_op("-", "-")
      .add_int("2", 2);
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..3)]);
    let elb = ElementListBuilder::default()
      .add_int("1", 1)
      .add_op("+", "+")
      .add_int("2", 2);
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..3)]);
  }
}
