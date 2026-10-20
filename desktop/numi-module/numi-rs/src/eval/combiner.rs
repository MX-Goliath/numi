use log::debug;

use crate::*;

/// Combines multipe evaluators based value priority, location
pub struct Combiner {
  pub evaluators: Vec<Box<dyn Evaluator>>,
}

impl Combiner {
  pub fn new(evaluators: Vec<Box<dyn Evaluator>>) -> Combiner {
    Combiner { evaluators }
  }
}

impl Evaluator for Combiner {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    let mut result = ElementList::new();
    for evaluator in &self.evaluators {
      if let Some(ev_list) = evaluator.evaluate(elements, context) {
        debug!("{}: {:?}", evaluator.type_name(), &ev_list);
        result.combine(ev_list);
      }
    }
    if !result.is_empty() {
      return result.some();
    }
    None
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::eval::NumberExtractor;
  use crate::op::Op;
  use crate::op::OpExtractor;
  use crate::op::OpRegistry;
  use rust_decimal_macros::dec;
  use std::cell::RefCell;
  use std::rc::Rc;

  #[test]
  fn test_single_evaluator() {
    let str = "a1";
    let input_el_list = ElementList::with_str(str);

    let range_detector = Rc::new(StrRanger::new());
    let number_extractor = NumberExtractor::new(range_detector);
    let combiner = Combiner::new(vec![Box::new(number_extractor)]);

    let result = combiner.evaluate(&input_el_list, &EvalContext::dumb());

    let expected = ElementList::with_elements(vec![Element::with_decimal_in_substr(
      dec!(1),
      Substr::new(str, 1..2),
    )]);

    assert_eq!(result.unwrap(), expected);
  }

  #[test]
  fn test_several_evaluators() {
    let str = "+1";
    let input_el_list = ElementList::with_str(str);

    let range_detector = Rc::new(StrRanger::new());
    let number_extractor = NumberExtractor::new(range_detector);
    let mut decimal_op = OpRegistry::default();
    decimal_op.add_op(Op::new("+"), None, vec!["+"]);
    let op_extractor = OpExtractor::new(Rc::new(RefCell::new(decimal_op)));
    let combiner = Combiner::new(vec![Box::new(number_extractor), Box::new(op_extractor)]);

    let result = combiner.evaluate(&input_el_list, &EvalContext::dumb());

    let expected = ElementList::with_elements(vec![
      Element::with_op_in_substr("+", Substr::new(str, 0..1)),
      Element::with_decimal_in_substr(dec!(1), Substr::new(str, 1..2)),
    ]);

    assert_eq!(result.unwrap(), expected);
  }

  // todo: test concurrent ops
}

impl<'a> ElementList<'a> {
  fn add_list(&mut self, other: ElementList<'a>) {
    for el in other.into_iter() {
      self.append(el);
    }
  }

  fn combine(&mut self, other: ElementList<'a>) {
    // todo: validate element list belong to the same str

    // Combine all elements and sort
    let mut sorted_els = self.elements.clone();
    sorted_els.extend(other.elements);
    sorted_els.sort_unstable_by(|a, b| {
      if a.value.priority() != b.value.priority() {
        return a
          .value
          .priority()
          .partial_cmp(&b.value.priority())
          .unwrap()
          .reverse();
      }

      let a_start = a.attr_text.range().start;
      let b_start = b.attr_text.range().start;
      a_start.partial_cmp(&b_start).unwrap().reverse()
    });

    // Filter out intersections
    let mut filtered_els: Vec<Element> = Vec::new();
    for el in sorted_els {
      let does_not_intersect = filtered_els.iter().all(|fel| {
        let fel_range = fel.attr_text.range();
        let el_range = el.attr_text.range();
        !fel_range.is_intersect(&el_range)
      });
      if does_not_intersect {
        filtered_els.push(el);
      }
    }

    // Sort by location
    filtered_els.sort_unstable_by(|a, b| {
      let a_start = a.attr_text.range().start;
      let b_start = b.attr_text.range().start;
      a_start.partial_cmp(&b_start).unwrap()
    });

    self.elements = filtered_els;
  }
}

#[cfg(test)]
mod element_list_tests {

  use super::*;
  use crate::Substr;

  #[test]
  fn test_combine_empty() {
    let mut result = ElementList::new();
    let other = ElementList::new();
    result.combine(other);
    assert!(result.is_empty());
  }

  #[test]
  fn test_combine_pref_other() {
    let str = "1";
    let mut result = ElementList::with_str(str);
    let mut el = Element::with_int_in_substr(5, Substr::new(str, 0..1));
    el.value.add_priority(2);
    let other = ElementList::with_element(el.clone());
    result.combine(other);
    let expected = ElementList::with_element(el);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_combine_non_overlap() {
    let str = "12";
    let e1 = Element::with_int_in_substr(1, Substr::new(str, 0..1));
    let e2 = Element::with_int_in_substr(2, Substr::new(str, 1..2));
    let other = ElementList::with_element(e2.clone());
    let mut result = ElementList::with_element(e1.clone());
    result.combine(other);
    let expected = ElementList::with_elements(vec![e1, e2]);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_combine_non_overlap_inverted() {
    let str = "12";
    let e1 = Element::with_int_in_substr(1, Substr::new(str, 0..1));
    let e2 = Element::with_int_in_substr(2, Substr::new(str, 1..2));
    let other = ElementList::with_element(e1.clone());
    let mut result = ElementList::with_element(e2.clone());
    result.combine(other);
    let expected = ElementList::with_elements(vec![e1, e2]);
    assert_eq!(result, expected);
  }
}
