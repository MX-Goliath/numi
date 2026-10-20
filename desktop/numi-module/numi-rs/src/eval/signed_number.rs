use rust_decimal_macros::dec;

use crate::matcher::{DecimalMatcher, FinderMatcher, OrMatcher, SequenceMatcher};
use crate::op::OpMatcher;
use crate::op::OpType;
use crate::*;

#[derive(Default)]
pub struct SignedNumberEval {
  matcher: SignedNumberMatcher,
}

impl Evaluator for SignedNumberEval {
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
      let op_value = &result.get(m.range.start).unwrap().value;
      let mut value = result.get(m.range.start + 1).unwrap().value.clone();
      if let Some(num) = value.number() {
        if op_value.op().unwrap() == "-" {
          value.add_number(dec!(0) - num);
        }
      }
      result.replace_elements_with_value(m.range, value)
    }

    result.some()
  }
}

#[cfg(test)]
mod signed_number_eval_test {
  use super::*;

  #[test]
  fn test_negative() {
    let eval = SignedNumberEval::default();
    let list_b = ElementList::builder().add_op("-", "-").add_int("1", 1);
    let result = eval
      .evaluate(&list_b.element_list(), &EvalContext::dumb())
      .unwrap();
    let expected_value = Value::default().with_int(-1);
    assert_eq!(
      result,
      Element::new(expected_value, list_b.element_list().attr_text().unwrap()).into_list()
    );
  }

  #[test]
  fn test_positive() {
    let eval = SignedNumberEval::default();
    let list_b = ElementList::builder().add_op("+", "+").add_int("1", 1);
    let result = eval
      .evaluate(&list_b.element_list(), &EvalContext::dumb())
      .unwrap();
    let expected_value = Value::default().with_int(1);
    assert_eq!(
      result,
      Element::new(expected_value, list_b.element_list().attr_text().unwrap()).into_list()
    );
  }
}

pub struct SignedNumberMatcher {
  matcher: FinderMatcher,
}

impl Default for SignedNumberMatcher {
  fn default() -> Self {
    let ops_matcher = OrMatcher::with_matchers(vec![
      OpMatcher::with_op(OpType::Some("-".to_string())).boxed(),
      OpMatcher::with_op(OpType::Some("+".to_string())).boxed(),
    ]);
    Self {
      matcher: FinderMatcher::with_matcher(
        SequenceMatcher::with_matchers(vec![ops_matcher.boxed(), DecimalMatcher::new().boxed()])
          .boxed(),
      ),
    }
  }
}

impl Matcher for SignedNumberMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    self.matcher.matches(element_list, from)
  }
}

#[cfg(test)]
mod test_negative_number_matcher {
  use super::*;
  use crate::element_list_builder::ElementListBuilder;

  #[test]
  fn test_negative_number() {
    let matcher = SignedNumberMatcher::default();
    let elb = ElementListBuilder::default()
      .add_op("-", "-")
      .add_int("1", 1);
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..2)]);
  }

  #[test]
  fn test_positive_number() {
    let matcher = SignedNumberMatcher::default();
    let elb = ElementListBuilder::default()
      .add_op("+", "+")
      .add_int("1", 1);
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..2)]);
  }
}
