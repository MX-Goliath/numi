use std::cell::RefCell;
use std::collections::HashMap;

use crate::date::DateTime;
use crate::ElementList;
use crate::Evaluator;
use crate::Value;

#[derive(Clone, Default)]
pub struct DataContext {
  data: RefCell<HashMap<String, DataContextValue>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum DataContextValue {
  SingleValue(Value),
  ValueMap(HashMap<String, Value>),
}

const VARIABLES_KEY: &str = "vars";
const DEFAULT_TIMEZONE_KEY: &str = "timezone";
const CURRENT_DATETIME_KEY: &str = "datetime";

impl DataContext {
  pub fn set_value(&self, key: &str, value: &Value) {
    self.data.borrow_mut().insert(
      key.to_string(),
      DataContextValue::SingleValue(value.clone()),
    );
  }

  // Return empty value if no key just to make a bit more readable for consumer
  pub fn value(&self, key: &str) -> Value {
    if let Some(DataContextValue::SingleValue(v)) = self.data.borrow().get(key) {
      return v.clone();
    }
    Value::default()
  }

  pub fn set_variable(&self, var_name: &str, value: &Value) {
    let mut map: HashMap<String, Value> = HashMap::new();
    if let Some(DataContextValue::ValueMap(vmap)) = self.data.borrow_mut().remove(VARIABLES_KEY) {
      map = vmap;
    }
    map.insert(var_name.to_string(), value.clone());
    self
      .data
      .borrow_mut()
      .insert(VARIABLES_KEY.to_string(), DataContextValue::ValueMap(map));
  }

  pub fn get_variable(&self, var_name: &str) -> Value {
    if let Some(DataContextValue::ValueMap(map)) = self.data.borrow().get(VARIABLES_KEY) {
      if let Some(v) = map.get(var_name) {
        return v.clone();
      }
    }
    Value::default()
  }

  pub fn all_variables(&self) -> Vec<String> {
    let data = self.data.borrow();
    let var_map = data.get(VARIABLES_KEY);
    if let Some(DataContextValue::ValueMap(map)) = var_map {
      return map.keys().map(|k| k.as_str().to_string()).collect();
    }
    vec![]
  }
}

impl DataContext {
  pub fn set_int_value(&self, key: &str, value: isize) {
    self.set_value(key, &Value::empty().with_int(value));
  }

  pub fn int_value(&self, key: &str) -> Option<isize> {
    self.value(key).int()
  }

  pub fn set_default_timezone(&mut self, timezone: &str) {
    self.set_value(
      DEFAULT_TIMEZONE_KEY,
      &Value::default().with_timezone(timezone),
    );
  }

  pub fn default_timezone(&self) -> Option<String> {
    self
      .value(DEFAULT_TIMEZONE_KEY)
      .timezone()
      .map(|s| s.to_string())
  }

  pub fn set_current_datetime(&mut self, time: DateTime) {
    self.set_value(CURRENT_DATETIME_KEY, &Value::default().with_datetime(time));
  }

  pub fn current_datetime(&self) -> Option<DateTime> {
    self.value(CURRENT_DATETIME_KEY).datetime()
  }

  const EVAL_LEVEL_KEY: &'static str = "eval_level";

  pub fn set_eval_level(&self, level: isize) {
    self.set_int_value(Self::EVAL_LEVEL_KEY, level);
  }

  pub fn eval_level(&self) -> isize {
    self.int_value(Self::EVAL_LEVEL_KEY).unwrap_or(0)
  }
}

pub struct EvalContext<'a> {
  pub eval: &'a dyn Evaluator, // Weak ?
  pub data: DataContext,
}

impl<'a> EvalContext<'a> {
  pub fn new(eval: &'a dyn Evaluator, data: DataContext) -> Self {
    EvalContext { eval, data }
  }

  pub fn dumb() -> Self {
    Self {
      eval: &DumbEvaluator {},
      data: DataContext::default(),
    }
  }
}

// This evaluatator used temprorarily just to make `EvalContext` compile
pub struct DumbEvaluator {}

impl Evaluator for DumbEvaluator {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    _context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    elements.clone().some()
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test_variables() {
    let data = DataContext::default();
    assert_eq!(data.get_variable("a"), Value::default());
    data.set_variable("a", &Value::empty().with_int(3));
    assert_eq!(data.get_variable("a"), Value::empty().with_int(3));
    assert_eq!(data.all_variables(), vec!["a"]);
  }

  #[test]
  fn test_eval_level() {
    let data = DataContext::default();
    assert_eq!(data.eval_level(), 0);
    data.set_eval_level(1);
    assert_eq!(data.eval_level(), 1);
  }
}
