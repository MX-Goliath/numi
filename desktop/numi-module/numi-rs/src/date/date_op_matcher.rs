use crate::matcher::*;
use crate::*;
use op::{OpMatcher, OpType};

pub struct DateOpMatcher {
  matcher: MatcherBox,
}

impl DateOpMatcher {
  pub fn with_ops(ops: Vec<&str>) -> Self {
    let mut op_matchers: Vec<Box<dyn Matcher>> = vec![];
    for op_id in ops.iter() {
      op_matchers.push(OpMatcher::with_op(OpType::Some(op_id.to_string())).boxed());
    }
    Self {
      matcher: SequenceMatcher::with_matchers(vec![
        DateMatcher::new().boxed(),
        OrMatcher::with_matchers(op_matchers).boxed(),
        DecimalMatcher::new().boxed(),
      ])
      .boxed(),
    }
  }
}

impl Matcher for DateOpMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    self.matcher.matches(element_list, from)
  }
}

#[cfg(test)]
mod date_op_matcher_test {

  use super::*;
  use crate::element_list_builder::ElementListBuilder;
  use date::DateTime;

  #[test]
  fn test() {
    let matcher = DateOpMatcher::with_ops(vec!["+", "-"]);
    let elb = ElementListBuilder::default()
      .add_date(
        "2021-01-01",
        DateTime::with_str_datetime("2021-01-01 00:00:00").unwrap(),
      )
      .add_op("+", "+")
      .add_int("2", 2);
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..3)]);
  }
}
