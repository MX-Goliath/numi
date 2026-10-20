use crate::*;

use std::{cell::RefCell, rc::Rc};

pub struct UnitExtractor {
  rc_ref_unit_manager: Rc<RefCell<UnitManager>>,
}

impl UnitExtractor {
  pub fn new(rc_ref_unit_manager: Rc<RefCell<UnitManager>>) -> Self {
    Self {
      rc_ref_unit_manager,
    }
  }
}

impl Evaluator for UnitExtractor {
  fn evaluate<'el>(
    &self,
    elements: &ElementList<'el>,
    _context: &EvalContext,
  ) -> Option<ElementList<'el>> {
    if let Some(first_el) = elements.first() {
      let substr = first_el.attr_text.substr.clone();
      let u_manager = self.rc_ref_unit_manager.borrow();
      let matches = u_manager.matches(substr);
      let res_elements: Vec<Element> = matches
        .into_iter()
        .map(|m| {
          return Element::new(
            Value::default().with_unit(m.id).with_priority(10),
            AttrText::with_substr_of_kind(m.substr, AttrKind::Unit),
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
mod tests {

  use crate::unit_manager::Unit;

  use super::*;

  #[test]
  fn test_percentage() {
    let mut unit_manager = UnitManager::default();
    unit_manager.add_unit(
      Unit {
        id: "%".to_string(),
        format: "%".to_string(),
      },
      &vec!["percent", "%"],
    );
    let rc_unit_manager = Rc::new(RefCell::new(unit_manager));
    let evaluator = UnitExtractor::new(rc_unit_manager);
    let str = "1%";
    let input_el_list = ElementList::with_str(str);
    let result = evaluator.evaluate(&input_el_list, &EvalContext::dumb());
    let el = Element::new(
      Value::default().with_unit("%").with_priority(10),
      AttrText::with_substr_of_kind(Substr::new(str, 1..2), AttrKind::Unit),
    );
    assert_eq!(result.unwrap(), ElementList::with_element(el));
  }
}
