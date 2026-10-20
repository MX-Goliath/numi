use std::{collections::HashMap, rc::Rc};

use crate::{
  eval_context::DataContext,
  fns::{Func, FuncId},
  phrase_detector::{PhraseDetector, PhraseMatch},
  substr::Substr,
  Value,
};

#[derive(Default)]
pub struct FuncManager {
  funcs: HashMap<FuncId, Rc<dyn Func>>,
  detector: PhraseDetector,
}

impl FuncManager {
  pub fn add_func(&mut self, id: &str, phrases: &Vec<&str>, func: Rc<dyn Func>) {
    self.detector.add_phrases_for(id, phrases);
    self.funcs.insert(id.to_string(), func);
  }

  pub fn evaluate_func(&self, id: &str, values: &[Value], context: &DataContext) -> Option<Value> {
    if let Some(func) = self.funcs.get(id) {
      return func.evaluate(id, values, context);
    }
    None
  }

  pub fn matches<'a, 's, S>(&'a self, into_substr: S) -> Vec<PhraseMatch<'a, 's>>
  where
    S: Into<Substr<'s>>,
  {
    self.detector.matches(into_substr)
  }
}
