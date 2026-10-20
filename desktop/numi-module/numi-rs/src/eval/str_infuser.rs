use crate::*;

pub struct StrInfuser {}

/// Infuse attributed text from elements without value attributes into following next elements with some value attributes
/// Used to reduce amount of pure text elements, which are not supposed to affect calculations
impl StrInfuser {
  pub fn new() -> StrInfuser {
    StrInfuser {}
  }
}

impl Evaluator for StrInfuser {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    _context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    if elements.is_empty() {
      return None;
    }
    let first_el = elements.first().unwrap();
    let first_str = first_el.attr_text.whole_str();
    let substr = Substr::with_empty_str(first_str);
    let mut acc_attr_text = AttrText::with_substr(substr);
    let mut result_els = Vec::new();

    for el in elements.clone().into_iter() {
      let value = el.value;
      acc_attr_text.merge_serial(el.attr_text);
      if value.has_attributes() {
        let el = Element::new(value, acc_attr_text);
        result_els.push(el);
        acc_attr_text = AttrText::with_substr(Substr::with_empty_str(first_str));
      }
    }

    if !acc_attr_text.is_empty() {
      if !result_els.is_empty() {
        let mut last_el = result_els.pop().unwrap();
        last_el.attr_text.merge_serial(acc_attr_text);
        result_els.push(last_el);
      } else {
        let str_el = Element::with_attr_text(acc_attr_text);
        result_els.push(str_el);
      }
    }

    ElementList::with_elements(result_els).some()
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_no_str_elements() {
    let infuser = StrInfuser::new();
    let input = ElementList::with_element(Element::with_int_in_str(1, "1"));
    let expected = input.clone();
    let result = infuser.evaluate(&input, &EvalContext::dumb());
    assert_eq!(result.unwrap(), expected);
  }

  #[test]
  fn test_just_str_elements() {
    let infuser = StrInfuser::new();
    let input = ElementList::with_str("a");
    let expected = input.clone();
    let result = infuser.evaluate(&input, &EvalContext::dumb());
    assert_eq!(result.unwrap(), expected);
  }

  #[test]
  fn test_infuse_prefix() {
    let str = "a1";

    let input = ElementList::with_elements(vec![
      Element::with_substr(Substr::new(str, 0..1)),
      Element::with_int_in_substr(1, Substr::new(str, 1..2)),
    ]);

    let infuser = StrInfuser::new();
    let expected_attr_str = AttrText::new(
      Substr::new(str, 0..2),
      vec![AttrEl::new(SRange::from_range(1..2), AttrKind::Number)],
    );
    let expected = Element::with_int_in_text(1, expected_attr_str).into_list();
    let result = infuser.evaluate(&input, &EvalContext::dumb());
    assert_eq!(result.unwrap(), expected);
  }
}
