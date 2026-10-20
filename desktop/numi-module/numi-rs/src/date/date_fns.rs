use crate::date::DateTime;
use crate::{fns::Func, Value};
use chrono::Local;

#[derive(Default)]
pub struct DateFns {}

impl Func for DateFns {
  fn evaluate(
    &self,
    fn_id: &str,
    _values: &[crate::Value],
    context: &crate::eval_context::DataContext,
  ) -> Option<crate::Value> {
    if fn_id == "now" {
      if let Some(datetime) = context.current_datetime() {
        return Some(Value::default().with_datetime(datetime));
      }
      let dtime = DateTime::new(Local::now().naive_local(), None);
      return Some(Value::default().with_datetime(dtime));
    }
    if fn_id == "today" {
      let dtime = DateTime::new(
        Local::now()
          .naive_local()
          .date()
          .and_hms_opt(0, 0, 0)
          .unwrap(),
        None,
      );
      return Some(Value::default().with_datetime(dtime));
    }
    if fn_id == "tomorrow" {
      let dtime = DateTime::new(
        Local::now()
          .naive_local()
          .date()
          .succ_opt()
          .unwrap()
          .and_hms_opt(0, 0, 0)
          .unwrap(),
        None,
      );
      return Some(Value::default().with_datetime(dtime));
    }
    if fn_id == "yesterday" {
      let dtime = DateTime::new(
        Local::now()
          .naive_local()
          .date()
          .pred_opt()
          .unwrap()
          .and_hms_opt(0, 0, 0)
          .unwrap(),
        None,
      );
      return Some(Value::default().with_datetime(dtime));
    }
    None
  }
}
