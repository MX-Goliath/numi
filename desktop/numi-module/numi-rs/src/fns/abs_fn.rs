use crate::{eval_context::DataContext, Value};

use super::Func;

#[derive(Default)]
pub struct AbsFunc {}

impl Func for AbsFunc {
  fn evaluate(&self, _fn_id: &str, values: &[Value], _context: &DataContext) -> Option<Value> {
    if let Some(v1) = values.first() {
      if let Some(num) = v1.number() {
        return Some(Value::default().with_number(num.abs()));
      }
    }
    None
  }
}

#[cfg(test)]
mod test_abs {

  use super::*;

  #[test]
  fn test() {
    let f = AbsFunc::default();
    assert_eq!(
      f.evaluate(
        "abs",
        &[Value::default().with_int(1)],
        &DataContext::default()
      )
      .unwrap(),
      Value::default().with_int(1)
    );
    assert_eq!(
      f.evaluate(
        "abs",
        &[Value::default().with_int(-1)],
        &DataContext::default()
      )
      .unwrap(),
      Value::default().with_int(1)
    );
  }
}
