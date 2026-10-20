use crate::*;

pub struct StrFiller {}

/// Fill non-sequential element list with strings for text ranges not covered by other elements
impl StrFiller {
  pub fn new() -> StrFiller {
    StrFiller {}
  }
}

impl Evaluator for StrFiller {
  fn evaluate<'list>(
    &self,
    list: &ElementList<'list>,
    _context: &EvalContext,
  ) -> Option<ElementList<'list>> {
    let mut result = ElementList::new();
    if let Some(str) = list.whole_str() {
      let mut insert_pos = 0;

      for el in list.iter() {
        let el_start = el.attr_text.range().start;
        if el_start > insert_pos {
          let substr = Substr::new(str, insert_pos..el_start);
          let element = Element::with_substr(substr);
          result.elements.push(element);
        }
        result.elements.push(el.clone());
        insert_pos = el.attr_text.range().end();
      }

      if insert_pos < str.len() {
        let substr = Substr::new(str, insert_pos..str.len());
        result.elements.push(Element::with_substr(substr))
      }
    }
    result.some()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_empty_list() {
    let filler = StrFiller {};
    let income_list = ElementList::new();
    let expected = ElementList::new();
    assert_eq!(
      filler.evaluate(&income_list, &EvalContext::dumb()).unwrap(),
      expected
    );
  }

  #[test]
  fn test_full_number() {
    let text = "123";
    let income = ElementList::with_element(Element::with_int_in_str(123, text));
    let expected = income.clone();
    let filler = StrFiller {};
    assert_eq!(
      filler.evaluate(&income, &EvalContext::dumb()).unwrap(),
      expected
    );
  }

  #[test]
  fn test_number_at_the_end() {
    let text = "a1";
    let income = ElementList::with_elements(vec![Element::with_int_in_substr(
      1,
      Substr::new(text, 1..2),
    )]);
    let expected = ElementList::with_elements(vec![
      Element::with_substr(Substr::new(text, 0..1)),
      Element::with_int_in_substr(1, Substr::new(text, 1..2)),
    ]);
    let filler = StrFiller {};
    assert_eq!(
      filler.evaluate(&income, &EvalContext::dumb()).unwrap(),
      expected
    );
  }

  #[test]
  fn test_number_at_the_beginning() {
    let text = "1a";
    let income = ElementList::with_elements(vec![Element::with_int_in_substr(
      1,
      Substr::new(text, 0..1),
    )]);
    let expected = ElementList::with_elements(vec![
      Element::with_int_in_substr(1, Substr::new(text, 0..1)),
      Element::with_substr(Substr::new(text, 1..2)),
    ]);
    let filler = StrFiller {};
    assert_eq!(
      filler.evaluate(&income, &EvalContext::dumb()).unwrap(),
      expected
    );
  }

  #[test]
  fn test_number_in_the_middle() {
    let text = "a1b";
    let income = ElementList::with_elements(vec![Element::with_int_in_substr(
      1,
      Substr::new(text, 1..2),
    )]);
    let expected = ElementList::with_elements(vec![
      Element::with_substr(Substr::new(text, 0..1)),
      Element::with_int_in_substr(1, Substr::new(text, 1..2)),
      Element::with_substr(Substr::new(text, 2..3)),
    ]);
    let filler = StrFiller {};
    assert_eq!(
      filler.evaluate(&income, &EvalContext::dumb()).unwrap(),
      expected
    );
  }
}
