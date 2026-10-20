use super::OpsMatcher;
use crate::op::OpType;
use crate::ElementList;
use crate::Match;
use crate::Matcher;

/// A matcher that identifies valid split points for binary operations.
/// This struct implements the `Matcher` trait to find appropriate locations
/// where an expression can be split based on binary operations. It filters out
/// invalid operation sequences (e.g., consecutive operations like in "5*-4")
/// and ensures that each matched operation has non-operation elements on both sides.
pub struct SplitOpMatcher {
  pub matcher: Box<dyn Matcher>,
}

impl SplitOpMatcher {
  pub fn with_operations(operations: Vec<OpType>) -> Self {
    let op_matcher = OpsMatcher::operations_finder(operations);
    Self {
      matcher: op_matcher.boxed(),
    }
  }
}

impl Matcher for SplitOpMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    let mut result = vec![];
    let ops_matches = self.matcher.matches(element_list, from);
    for m in ops_matches.iter() {
      if m.range.start > 0 && m.range.end < element_list.len() {
        let prev_op = element_list.get(m.range.start - 1).unwrap().value.has_op();
        if !prev_op {
          result.push(m.clone());
        }
      }
    }

    result
  }
}

#[cfg(test)]
mod test {

  use super::*;
  use crate::{element_list_builder::ElementListBuilder, *};

  #[test]
  fn test_split_ops_in_the_middle() {
    let elb = ElementListBuilder::default()
      .add_text("a")
      .add_op("+", "+")
      .add_op("-", "-")
      .add_text("c");
    let matcher =
      SplitOpMatcher::with_operations(vec![OpType::with_some("-"), OpType::with_some("+")]);
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_element_at(1)]);
  }

  #[test]
  fn test_split_ops_in_beginning() {
    let elb = ElementListBuilder::default().add_op("-", "-").add_text("c");
    let matcher =
      SplitOpMatcher::with_operations(vec![OpType::with_some("-"), OpType::with_some("+")]);
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![]);
  }

  #[test]
  fn test_split_ops_at_the_end() {
    let elb = ElementListBuilder::default().add_text("c").add_op("-", "-");
    let matcher =
      SplitOpMatcher::with_operations(vec![OpType::with_some("-"), OpType::with_some("+")]);
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![]);
  }
}
