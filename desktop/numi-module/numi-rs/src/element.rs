use crate::op::DEFAULT_OP_PRIORITY;

use super::*;
use rust_decimal::Decimal;
use std::ops::Range;

#[derive(PartialEq, Debug, Clone)]
pub struct Element<'a> {
  pub value: Value,
  pub attr_text: AttrText<'a>,
}

impl<'a> Element<'a> {
  // decimal

  pub fn with_decimal_range_in_str(decimal: Decimal, range: Range<usize>, str: &'a str) -> Self {
    let value = Value::empty().with_number(decimal);
    let attr_text = AttrText::with_substr_of_kind_in_range(str, AttrKind::Number, range);
    Self::new(value, attr_text)
  }

  pub fn with_decimal_in_str(decimal: Decimal, str: &'a str) -> Element {
    Self::with_decimal_in_substr(decimal, Substr::with_str(str))
  }

  pub fn with_decimal_in_substr(decimal: Decimal, substr: Substr<'a>) -> Element {
    let value = Value::default().with_number(decimal);
    let attr_text = AttrText::with_substr_of_kind(substr, AttrKind::Number);
    Self::new(value, attr_text)
  }

  // Int

  pub fn with_int_in_str(number: isize, str: &'a str) -> Element {
    Self::with_int_in_substr(number, Substr::with_str(str))
  }

  pub fn with_int_in_substr(number: isize, substr: Substr<'a>) -> Element {
    let value = Value::default().with_int(number);
    let attr_text = AttrText::with_substr_of_kind(substr, AttrKind::Number);
    Self::new(value, attr_text)
  }

  pub fn with_int_in_text(number: isize, attr_text: AttrText<'a>) -> Element {
    Self::new(Value::default().with_int(number), attr_text)
  }

  // ops

  pub fn with_op_in_substr<'b>(op: &'b str, substr: Substr<'a>) -> Element<'a> {
    let value = Value::empty()
      .with_op(op)
      .with_priority(DEFAULT_OP_PRIORITY);
    let attr_text = AttrText::with_substr_of_kind(substr, AttrKind::Op);
    Self::new(value, attr_text)
  }

  // text

  pub fn with_str(str: &'a str) -> Element<'a> {
    Self::with_substr(Substr::with_str(str))
  }

  pub fn with_substr(substr: Substr<'a>) -> Element<'a> {
    let value = Value::empty();
    let attr_text = AttrText::with_substr(substr);
    Self::new(value, attr_text)
  }

  pub fn with_attr_text(attr_text: AttrText<'a>) -> Element {
    Self::new(Value::empty(), attr_text)
  }

  // general

  pub fn into_list(self) -> ElementList<'a> {
    ElementList::with_element(self)
  }

  pub fn has_some_value(&self) -> bool {
    self.value.has_attributes()
  }

  pub fn new(value: Value, attr_text: AttrText<'a>) -> Element {
    Self { value, attr_text }
  }
}
