use log::debug;

use crate::ElementList;
use crate::EvalContext;
use crate::Evaluator;

pub struct ListEval {
  evals: Vec<Box<dyn Evaluator>>,
}

impl ListEval {
  pub fn with_evaluators(evaluators: Vec<Box<dyn Evaluator>>) -> ListEval {
    ListEval { evals: evaluators }
  }
}

impl Evaluator for ListEval {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    let current_level = context.data.eval_level();
    context.data.set_eval_level(current_level + 1);
    let mut result = elements.clone();
    for eval in &self.evals {
      if let Some(lres) = eval.evaluate(&result, context) {
        debug!("{}: {:?}", eval.type_name(), &lres);

        // return early if error
        if lres.has_error() {
          context.data.set_eval_level(current_level);
          return lres.some();
        }

        if !lres.is_empty() {
          result = lres;
        }
      }
    }
    context.data.set_eval_level(current_level);
    result.some()
  }
}
