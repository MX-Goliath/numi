use std::{cell::RefCell, rc::Rc};

const DEFAULT_FN_PRIORITY: isize = 102;

use crate::{
  fns::FuncManager, AttrKind, AttrText, Element, ElementList, EvalContext, Evaluator, Value,
};

pub struct FuncExtractor {
  func_manager: Rc<RefCell<FuncManager>>,
}

impl FuncExtractor {
  pub fn new(func_manager: Rc<RefCell<FuncManager>>) -> Self {
    Self { func_manager }
  }
}

impl Evaluator for FuncExtractor {
  fn evaluate<'el>(
    &self,
    elements: &ElementList<'el>,
    _context: &EvalContext,
  ) -> Option<ElementList<'el>> {
    if let Some(first_el) = elements.first() {
      let substr = first_el.attr_text.substr.clone();
      let f_manager = self.func_manager.borrow();
      let matches = f_manager.matches(substr);
      let res_elements: Vec<Element> = matches
        .into_iter()
        .map(|m| {
          return Element::new(
            Value::default()
              .with_func(m.id)
              .with_priority(DEFAULT_FN_PRIORITY),
            AttrText::with_substr_of_kind(m.substr, AttrKind::Func),
          );
        })
        .collect();
      ElementList::with_elements(res_elements).some()
    } else {
      None
    }
  }
}

#[cfg(test)]
mod test_func_extractor {
  use crate::{fns::AbsFunc, Substr};

  use super::*;

  #[test]
  fn test_func_extractor() {
    let mut func_manager = FuncManager::default();
    func_manager.add_func("abs", &vec!["abs"], Rc::<AbsFunc>::default());
    let rc_manager = Rc::new(RefCell::new(func_manager));
    let evaluator = FuncExtractor::new(rc_manager);
    let str = "abs 5";
    let input_el_list = ElementList::with_str(str);
    let result = evaluator.evaluate(&input_el_list, &EvalContext::dumb());
    let el = Element::new(
      Value::default()
        .with_func("abs")
        .with_priority(DEFAULT_FN_PRIORITY),
      AttrText::with_substr_of_kind(Substr::new(str, 0..3), AttrKind::Func),
    );
    assert_eq!(result.unwrap(), ElementList::with_element(el));
  }
}
