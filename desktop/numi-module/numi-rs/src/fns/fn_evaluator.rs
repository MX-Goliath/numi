use super::FuncExpressionMatcher;
use crate::matcher::Matcher;
use crate::{fns::FuncManager, ElementList, EvalContext, Evaluator, Value};
use std::{cell::RefCell, rc::Rc};

pub struct FuncEvaluator {
  matcher: FuncExpressionMatcher,
  func_manager: Rc<RefCell<FuncManager>>,
}

impl FuncEvaluator {
  pub fn new(func_manager: Rc<RefCell<FuncManager>>) -> Self {
    Self {
      matcher: FuncExpressionMatcher::default(),
      func_manager,
    }
  }
}

impl Evaluator for FuncEvaluator {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    let mut matches = self.matcher.matches(elements, 0);
    if matches.is_empty() {
      return None;
    }
    matches.reverse();
    let mut result = elements.clone();

    let fn_manager = self.func_manager.borrow();
    for m in matches {
      let fn_id = result.get(m.range.start).unwrap().value.func().unwrap();
      let mut args: Vec<Value> = vec![];
      // NOTE: add support for multi-arg functions
      if let Some(el) = result.get(m.range.start + 1) {
        args.push(el.value.clone());
      }
      if let Some(res_value) = fn_manager.evaluate_func(fn_id, &args, &context.data) {
        result.replace_elements_with_value(m.range, res_value);
      }
    }

    result.some()
  }
}

#[cfg(test)]
mod func_eval_test {
  use crate::{fns::AbsFunc, Element};

  use super::*;

  #[test]
  fn test() {
    let mut func_manager = FuncManager::default();
    func_manager.add_func("abs", &vec!["abs"], Rc::<AbsFunc>::default());
    let rc_manager = Rc::new(RefCell::new(func_manager));
    let eval = FuncEvaluator::new(rc_manager);
    let list_b = ElementList::builder()
      .add_func("abs", "abs")
      .add_int("-1", -1);
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
