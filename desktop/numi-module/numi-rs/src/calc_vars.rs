use crate::{Calc, Value};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

impl Calc {
  pub fn add_vars(&mut self) {
    self
      .data_context
      .set_variable("pi", &Value::default().with_number(Decimal::PI));
    self
      .data_context
      .set_variable("π", &Value::default().with_number(Decimal::PI));
    self
      .data_context
      .set_variable("e", &Value::default().with_number(Decimal::E));
    self
      .data_context
      .set_variable("r", &Value::default().with_number(dec!(8.3144598)));
  }
}
