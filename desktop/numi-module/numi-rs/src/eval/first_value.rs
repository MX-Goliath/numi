use crate::*;

pub struct FirstValue {}

//todo: Actually first el infuser
impl FirstValue {
  pub fn new() -> FirstValue {
    FirstValue {}
  }
}

impl Evaluator for FirstValue {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    _context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    if elements.is_empty() {
      return None;
    }
    if let Some(compound_attr_text) = elements.attr_text() {
      let mut result_el = elements.first().unwrap().clone();
      result_el.attr_text = compound_attr_text;
      return ElementList::with_element(result_el).some();
    }
    None
  }
}
