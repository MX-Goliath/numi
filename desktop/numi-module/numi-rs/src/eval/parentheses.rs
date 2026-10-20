use crate::op::OpType;
use crate::{
  element_list::ElementList,
  eval_context::EvalContext,
  matcher::{FinderMatcher, Match, Matcher},
  op::OpsMatcher,
  Value,
};

use super::Evaluator;
pub struct ParenthesesEvaluator {
  matcher: AllMatchingParenthesesMatcher,
}

impl ParenthesesEvaluator {
  pub fn new() -> Self {
    Self {
      matcher: AllMatchingParenthesesMatcher::new(),
    }
  }
}

impl Evaluator for ParenthesesEvaluator {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    let mut result = elements.clone();
    let mut matches = self.matcher.matches(elements, 0);
    matches.reverse();

    for m in matches {
      let inner_range = m.range.start + 1..m.range.end - 1;

      // Form result with parentheses and evaluated sublist

      let mut m_result = ElementList::empty();

      // Insert first element with empty value
      let mut first_el = elements.get(m.range.start).unwrap().clone();
      first_el.value = Value::empty();
      m_result.append(first_el);

      if !inner_range.is_empty() {
        if let Some(m_eval_result) = context
          .eval
          .evaluate(&elements.element_list_in(inner_range), context)
        {
          m_result.append_list(m_eval_result);
        }
      }

      // Insert last element with empty value
      let mut last_el = elements.get(m.range.end - 1).unwrap().clone();
      last_el.value = Value::empty();
      m_result.append(last_el);

      // now merges '(' and ')' with closest element

      let end_value = &m_result.get(m_result.len() - 2).unwrap().value;
      m_result
        .replace_elements_with_value((m_result.len() - 2)..(m_result.len()), end_value.clone());

      if m_result.len() > 1 {
        let first_value = &m_result.get(1).unwrap().value;
        m_result.replace_elements_with_value(0..2, first_value.clone());
      }

      result.replace_elements(m.range, m_result);
    }

    result.some()
  }
}

#[cfg(test)]
mod parentheses_evaluator_test {
  use super::*;
  use crate::{
    element::Element, element_list_builder::ElementListBuilder, eval_context::DataContext, Value,
  };
  use pretty_assertions::assert_eq;

  pub struct TestEvaluator {}

  impl Evaluator for TestEvaluator {
    /// Converts text in elements to element with number, containing number of characters
    fn evaluate<'a>(
      &self,
      elements: &ElementList<'a>,
      _context: &EvalContext,
    ) -> Option<ElementList<'a>> {
      let mut result = ElementList::new();
      if let Some(attr_text) = elements.attr_text() {
        let len = attr_text.str_in_range().len();
        result.append(Element::with_int_in_substr(len as isize, attr_text.substr));
      }
      result.some()
    }
  }

  #[test]
  fn test() {
    let eval = ParenthesesEvaluator::new();
    let elb = ElementListBuilder::default()
      .add_op("(", "(")
      .add_text("a")
      .add_op(")", ")");
    let source_list = elb.element_list();
    let eval_context = EvalContext::new(&TestEvaluator {}, DataContext::default());
    let result = eval.evaluate(&source_list, &eval_context);
    let expected_elb = ElementListBuilder::default()
      .add_op("(", "(")
      .add_int("a", 1)
      .add_op(")", ")");
    let expected_el = Element::new(
      Value::default().with_int(1),
      expected_elb.element_list().attr_text().unwrap(),
    );
    let expected_list = ElementList::with_element(expected_el);
    assert_eq!(result.unwrap(), expected_list);
  }

  #[test]
  fn empty_parentheses() {
    let eval = ParenthesesEvaluator::new();
    let elb = ElementListBuilder::default()
      .add_op("(", "(")
      .add_op(")", ")");
    let source_list = elb.element_list();
    let eval_context = EvalContext::new(&TestEvaluator {}, DataContext::default());
    let result = eval.evaluate(&source_list, &eval_context);
    let expected_el = Element::with_attr_text(elb.element_list().attr_text().unwrap());
    let expected_list = ElementList::with_element(expected_el);
    assert_eq!(result.unwrap(), expected_list);
  }
}

pub struct AllMatchingParenthesesMatcher {
  matcher: FinderMatcher,
}

impl AllMatchingParenthesesMatcher {
  fn new() -> Self {
    Self {
      matcher: FinderMatcher::with_matcher(MatchingParenthesesMatcher {}.boxed()),
    }
  }
}

impl Matcher for AllMatchingParenthesesMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    self.matcher.matches(element_list, from)
  }
}

pub struct MatchingParenthesesMatcher {}

impl Matcher for MatchingParenthesesMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<crate::matcher::Match> {
    let m_parentheses = OpsMatcher::operations_finder(vec![
      OpType::Some("(".to_owned()),
      OpType::Some(")".to_owned()),
    ]);
    let ms = m_parentheses.matches(element_list, from);

    // At least 2 parentheses required, should start from `from`
    if ms.len() < 2 || ms.first().unwrap().range.start != from {
      return vec![];
    }

    let mut level = 0;
    for m in ms {
      let op = element_list.get(m.range.start).unwrap().value.op().unwrap();
      if op == "(" {
        level += 1;
      }
      if op == ")" {
        level -= 1;
      }
      if level < 0 {
        return vec![];
      }
      if level == 0 {
        return vec![Match::with_range(from..m.range.end)];
      }
    }

    vec![]
  }
}

#[cfg(test)]
mod all_matching_parentheses_test {

  use crate::element_list_builder::ElementListBuilder;

  use super::*;

  #[test]
  fn test() {
    let elb = ElementListBuilder::default()
      .add_op("(", "(")
      .add_op(")", ")");
    let matcher = AllMatchingParenthesesMatcher::new();
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..2)]);
  }

  #[test]
  fn test_few() {
    let elb = ElementListBuilder::default()
      .add_op("(", "(")
      .add_op(")", ")")
      .add_op("(", "(")
      .add_op(")", ")");
    let matcher = AllMatchingParenthesesMatcher::new();
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(
      matches,
      vec![Match::with_range(0..2), Match::with_range(2..4)]
    );
  }
}

#[cfg(test)]
mod matching_parentheses_matcher_test {
  use super::*;
  use crate::{element_list_builder::ElementListBuilder, matcher::Match};

  #[test]
  fn test() {
    let elb = ElementListBuilder::default()
      .add_op("(", "(")
      .add_text("a")
      .add_op(")", ")");
    let matcher = MatchingParenthesesMatcher {};
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..3)]);
  }

  #[test]
  fn test_double_parentheses() {
    let elb = ElementListBuilder::default()
      .add_op("(", "(")
      .add_op("(", "(")
      .add_op(")", ")")
      .add_op(")", ")");
    let matcher = MatchingParenthesesMatcher {};
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..4)]);
  }

  #[test]
  fn test_closed_on_start() {
    let elb = ElementListBuilder::default()
      .add_op(")", ")")
      .add_op("(", "(")
      .add_op(")", ")");
    let matcher = MatchingParenthesesMatcher {};
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![]);
  }

  #[test]
  fn test_multiple_ranges() {
    let elb = ElementListBuilder::default()
      .add_op("(", "(")
      .add_op(")", ")")
      .add_op("(", "(")
      .add_op(")", ")");
    let matcher = MatchingParenthesesMatcher {};
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..2)]);
  }
}
