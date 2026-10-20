use crate::{
  element::Element,
  element_list::ElementList,
  eval_context::EvalContext,
  matcher::{Match, Matcher},
};

use super::Evaluator;

/// Generic evaluator that just accepts matcher and weight fn.
/// Should split element list based on some matcher (run finder on all possible matches)
/// Use weight Fn to control order if needed, whichever is smaller will be splitted first
/// giving a chance for a left and right parts to be evaluated earlier, recursively if needed
/// For the same weight evaluate from the end, for the same purpose: split and evaluate left/right
/// parts earlier.
pub struct SplitEvaluator<'a> {
  matcher: Box<dyn Matcher>,
  weight_fn: Box<dyn Fn(&Element) -> isize + 'a>,
}

impl<'a> SplitEvaluator<'a> {
  pub fn new<F>(matcher: Box<dyn Matcher>, closure: F) -> Self
  where
    F: Fn(&Element) -> isize + 'a,
  {
    Self {
      matcher,
      weight_fn: Box::new(closure),
    }
  }
}

impl Evaluator for SplitEvaluator<'_> {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    let mut matches = self.matcher.matches(elements, 0);
    // We should reverse the order so the last match should be splitted first
    // thus giving a chance to evaluate earlier matches earlie,
    // i.e. in case of "20-5+1" we should split on "+" first, and evaluate "20-5"
    // as a first operation, otherwise it will evaluate last operation first
    // and result will be incorrect: "5+1" and then "20-6"
    matches.reverse();

    // Similar strategy for weight, we should split at the lowest possible weight first
    // So overall split at the rightmost lowest weight
    let mut split_match: Option<Match> = Option::None;
    let mut match_weight = isize::MAX;

    // Looking for lightest rightmost element for a split
    for m in matches.iter() {
      let el_index = m.range.start;
      if let Some(el) = elements.get(el_index) {
        let m_weight = (self.weight_fn)(el);
        if m_weight < match_weight {
          split_match = Some(m.clone());
          match_weight = m_weight;
        }
      }
    }

    let m = split_match?;
    let split_at = m.range.start;
    if let Some(split_el) = elements.get(split_at) {
      let left_elements = elements.element_list_to(split_at);
      let right_elements = elements.element_list_from(split_at + 1);

      let mut result = ElementList::new();

      if !left_elements.is_empty() {
        if let Some(left_results) = context.eval.evaluate(&left_elements, context) {
          result.append_list(left_results);
        }
      }

      result.append(split_el.clone());

      if !right_elements.is_empty() {
        if let Some(right_results) = context.eval.evaluate(&right_elements, context) {
          result.append_list(right_results);
        }
      }

      return result.some();
    };

    None
  }
}

#[cfg(test)]
mod split_evaluator_test {

  use super::*;
  use crate::{
    element::Element, element_list_builder::ElementListBuilder, eval_context::DataContext,
    matcher::TestMatcher,
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
  fn test_single_element() {
    let split_eval = SplitEvaluator {
      matcher: TestMatcher::with_match_range(1..2).boxed(),
      weight_fn: Box::new(|_el: &Element| 0),
    };
    let elb = ElementListBuilder::default()
      .add_text("a")
      .add_text("bb")
      .add_text("ccc");
    let source_list = elb.element_list();
    let eval_context = EvalContext::new(&TestEvaluator {}, DataContext::default());
    let result = split_eval.evaluate(&source_list, &eval_context);
    let expected_elb = ElementListBuilder::default()
      .add_int("a", 1)
      .add_text("bb")
      .add_int("ccc", 3);
    let expected_list = expected_elb.element_list();
    assert_eq!(result.unwrap(), expected_list);
  }

  #[test]
  fn test_multi_element_list() {
    let split_eval = SplitEvaluator {
      matcher: TestMatcher::with_match_range(2..3).boxed(),
      weight_fn: Box::new(|_el: &Element| 0),
    };
    let elb = ElementListBuilder::default()
      .add_text("a")
      .add_text("bb")
      .add_text("ccc")
      .add_text("dddd")
      .add_text("eeeee");
    let source_list = elb.element_list();
    let eval_context = EvalContext::new(&TestEvaluator {}, DataContext::default());
    let result = split_eval.evaluate(&source_list, &eval_context);
    let expected_elb = ElementListBuilder::default()
      .add_int("abb", 3)
      .add_text("ccc")
      .add_int("ddddeeeee", 9);
    let expected_list = expected_elb.element_list();
    assert_eq!(result.unwrap(), expected_list);
  }

  #[test]
  fn test_no_before() {
    let split_eval = SplitEvaluator {
      matcher: TestMatcher::with_match_range(0..1).boxed(),
      weight_fn: Box::new(|_el: &Element| 0),
    };
    let elb = ElementListBuilder::default().add_text("a").add_text("bb");
    let source_list = elb.element_list();
    let eval_context = EvalContext::new(&TestEvaluator {}, DataContext::default());
    let result = split_eval.evaluate(&source_list, &eval_context);
    let expected_elb = ElementListBuilder::default().add_text("a").add_int("bb", 2);
    let expected_list = expected_elb.element_list();
    assert_eq!(result.unwrap(), expected_list);
  }

  #[test]
  fn test_no_after() {
    let split_eval = SplitEvaluator {
      matcher: TestMatcher::with_match_range(1..2).boxed(),
      weight_fn: Box::new(|_el: &Element| 0),
    };
    let elb = ElementListBuilder::default().add_text("a").add_text("bb");
    let source_list = elb.element_list();
    let eval_context = EvalContext::new(&TestEvaluator {}, DataContext::default());
    let result = split_eval.evaluate(&source_list, &eval_context);
    let expected_elb = ElementListBuilder::default().add_int("a", 1).add_text("bb");
    let expected_list = expected_elb.element_list();
    assert_eq!(result.unwrap(), expected_list);
  }
}
