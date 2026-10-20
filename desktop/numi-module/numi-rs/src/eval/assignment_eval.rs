use crate::op::OpType;
use crate::{matcher::*, AttrKind, Value};
use crate::{
  matcher::{AnyElementMatcher, GroupMatcher, RepeatMatcher, SequenceMatcher},
  op::OpMatcher,
  Evaluator,
};

#[derive(Default)]
pub struct AssignmentEval {
  matcher: AssignmentMatcher,
}

impl Evaluator for AssignmentEval {
  fn evaluate<'a>(
    &self,
    elements: &crate::ElementList<'a>,
    context: &crate::EvalContext,
  ) -> Option<crate::ElementList<'a>> {
    let matches = self.matcher.matches(elements, 0);
    if matches.is_empty() {
      return None;
    }
    let mut result = elements.clone();

    if let Some(m) = matches.first() {
      let var_name_range = m.groups.get(&1).unwrap()[0].clone();
      let expression_range = m.groups.get(&2).unwrap()[0].clone();
      let var_name = elements
        .attr_text_in_range(var_name_range.clone())
        .unwrap()
        .substr
        .str_in_range()
        .trim();

      if let Some(m_eval_result) = context
        .eval
        .evaluate(&elements.element_list_in(expression_range.clone()), context)
      {
        let result_value = m_eval_result.first().unwrap().value.clone();
        context.data.set_variable(var_name, &result_value);
        result.replace_elements_with_value(expression_range.clone(), Value::default());
        result.replace_elements_with_attr_value(
          var_name_range.clone(),
          AttrKind::Var,
          Value::default(),
        );
        result.replace_elements_with_value(
          var_name_range.clone().start..var_name_range.clone().start + 3,
          result_value,
        )
      }
    }

    result.some()
  }
}

#[cfg(test)]
mod assignment_evaluator_test {
  use super::*;
  use crate::{
    element::Element, element_list_builder::ElementListBuilder, eval_context::DataContext,
    ElementList, EvalContext, Value,
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
    let eval = AssignmentEval::default();
    let elb = ElementListBuilder::default()
      .add_text("a")
      .add_op("=", "=")
      .add_text("1");
    let source_list = elb.element_list();
    let eval_context = EvalContext::new(&TestEvaluator {}, DataContext::default());
    let result = eval.evaluate(&source_list, &eval_context);
    let result_value = result.unwrap().first().unwrap().value.clone();
    let expected_value = Value::default().with_int(1);
    assert_eq!(result_value, expected_value);
    assert_eq!(eval_context.data.get_variable("a"), expected_value);
  }

  #[test]
  fn test_whitespace() {
    let eval = AssignmentEval::default();
    let elb = ElementListBuilder::default()
      .add_text(" a ")
      .add_op("=", "=")
      .add_text("xxx");
    let source_list = elb.element_list();
    let eval_context = EvalContext::new(&TestEvaluator {}, DataContext::default());
    let result = eval.evaluate(&source_list, &eval_context);
    let result_value = result.unwrap().first().unwrap().value.clone();
    let expected_value = Value::default().with_int(3);
    assert_eq!(result_value, expected_value);
    assert_eq!(eval_context.data.get_variable("a"), expected_value);
  }
}

pub struct AssignmentMatcher {
  matcher: SequenceMatcher,
}

impl Default for AssignmentMatcher {
  fn default() -> Self {
    Self {
      matcher: SequenceMatcher::with_matchers(vec![
        GroupMatcher::new(
          1,
          RepeatMatcher::new(AnyElementMatcher::new().boxed(), Some(1)).boxed(),
        )
        .boxed(),
        OpMatcher::with_op(OpType::Some("=".to_string())).boxed(),
        GroupMatcher::new(
          2,
          RepeatMatcher::new(AnyElementMatcher::new().boxed(), Some(1)).boxed(),
        )
        .boxed(),
      ]),
    }
  }
}

impl Matcher for AssignmentMatcher {
  fn matches(&self, element_list: &crate::element_list::ElementList, from: isize) -> Vec<Match> {
    // repeat matcher return all combinations for expression,
    // we interested in max (whole thing)
    let matches = self.matcher.matches(element_list, from);
    // dbg!(element_list);
    let max_length = matches.iter().map(|m| m.range.len()).max().unwrap_or(0);
    matches
      .into_iter()
      .filter(|m| m.range.len() == max_length)
      .collect()
  }
}

#[cfg(test)]
mod assignment_matcher_test {

  use super::*;
  use crate::{
    element_list_builder::ElementListBuilder,
    matcher::{Match, MatchGroups},
  };

  #[test]
  fn test_simple_assignment() {
    let matcher = AssignmentMatcher::default();
    let elb = ElementListBuilder::default()
      .add_text("a")
      .add_op("=", "=")
      .add_int("3", 3);
    let matches = matcher.matches(&elb.element_list(), 0);
    let mut mg = MatchGroups::new();
    mg.insert(1, vec![0..1]);
    mg.insert(2, vec![2..3]);
    assert_eq!(matches, vec![Match::new(0..3, mg)]);
  }

  #[test]
  fn test_multi_el_assignment() {
    let matcher = AssignmentMatcher::default();
    let elb = ElementListBuilder::default()
      .add_text("a")
      .add_text("b")
      .add_op("=", "=")
      .add_int("3", 3)
      .add_op("+", "+")
      .add_int("4", 4);
    let matches = matcher.matches(&elb.element_list(), 0);
    let mut mg = MatchGroups::new();
    mg.insert(1, vec![0..2]);
    mg.insert(2, vec![3..6]);
    assert_eq!(matches, vec![Match::new(0..6, mg)]);
  }
}
