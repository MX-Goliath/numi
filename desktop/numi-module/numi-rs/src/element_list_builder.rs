use super::*;
use crate::{op::DEFAULT_OP_PRIORITY, value::ReprType};
use date::DateTime;
use rust_decimal::Decimal;
use std::ops::Range;

/// Helper struct for element list construction
#[derive(PartialEq, Debug, Clone, Default)]
pub struct ElementListBuilder {
  pub string: String,
  elements: Vec<ElbElement>,
}

impl ElementListBuilder {
  fn new(string: String, elements: Vec<ElbElement>) -> Self {
    ElementListBuilder { string, elements }
  }

  pub fn add_text(self, str: &str) -> Self {
    self.add_value_of_kind(str, Value::empty(), None)
  }

  pub fn add_text_of_kind(self, str: &str, kind: AttrKind) -> Self {
    self.add_value_of_kind(str, Value::empty(), Some(kind))
  }

  pub fn add_int(self, str: &str, number: isize) -> Self {
    self.add_decimal(str, Decimal::new(number as i64, 0))
  }

  pub fn add_int_unit(self, str: &str, number: isize, unit: &str) -> Self {
    self.add_value_of_kind(
      str,
      Value::empty().with_int(number).with_unit(unit),
      Some(AttrKind::Number),
    )
  }

  pub fn add_decimal(self, str: &str, decimal: Decimal) -> Self {
    self.add_value_of_kind(
      str,
      Value::empty().with_number(decimal),
      Some(AttrKind::Number),
    )
  }

  pub fn add_date(self, str: &str, date: DateTime) -> Self {
    self.add_value_of_kind(
      str,
      Value::empty().with_datetime(date),
      Some(AttrKind::Number),
    )
  }

  pub fn add_timezone(self, str: &str, tz: &str) -> Self {
    self.add_value_of_kind(str, Value::empty().with_timezone(tz), Some(AttrKind::Unit))
  }

  pub fn add_decimal_with_repr(self, str: &str, decimal: Decimal, repr: ReprType) -> Self {
    self.add_value_of_kind(
      str,
      Value::empty().with_number(decimal).with_repr(repr),
      Some(AttrKind::Number),
    )
  }

  pub fn with_priority(mut self, priority: isize) -> Self {
    let mut last_el = self.elements.pop().unwrap();
    last_el.value.add_priority(priority);
    self.elements.push(last_el);
    self
  }

  pub fn add_op(self, str: &str, op: &str) -> Self {
    self.add_value_of_kind(
      str,
      Value::empty()
        .with_op(op)
        .with_priority(DEFAULT_OP_PRIORITY),
      Some(AttrKind::Op),
    )
  }

  pub fn add_unit(self, str: &str, unit: &str) -> Self {
    self.add_value_of_kind(str, Value::default().with_unit(unit), Some(AttrKind::Unit))
  }

  pub fn add_func(self, str: &str, func_id: &str) -> Self {
    self.add_value_of_kind(
      str,
      Value::default().with_func(func_id),
      Some(AttrKind::Func),
    )
  }

  pub fn add_scale(self, str: &str, scale_id: &str) -> Self {
    self.add_value_of_kind(
      str,
      Value::default().with_scale(scale_id),
      Some(AttrKind::Unit),
    )
  }

  pub fn add_repr(self, str: &str, repr: ReprType) -> Self {
    self.add_value_of_kind(str, Value::default().with_repr(repr), Some(AttrKind::Unit))
  }

  pub fn add_value_of_kind(mut self, str: &str, value: Value, attr_kind: Option<AttrKind>) -> Self {
    let str_range = SRange::new(self.string.len(), str.len());
    let mut vec_markup = vec![];
    if let Some(attr_kind) = attr_kind {
      vec_markup.push(AttrEl::new(str_range, attr_kind));
    }
    let element = ElbElement::new(value, str_range, vec_markup);
    self.string.push_str(str);
    self.elements.push(element);
    self
  }

  pub fn int_at(&self, number: isize, at: usize) -> Element {
    let start = self.elements[at].range.start;
    self.int_in_range(number, start..start + 1)
  }

  pub fn int_in_range(&self, number: isize, range: Range<usize>) -> Element {
    Element::with_int_in_substr(number, Substr::new(&self.string, range))
  }

  pub fn element_list(&self) -> ElementList {
    self.element_list_in_range(0..self.elements.len())
  }

  pub fn element_list_in_range(&self, range: Range<usize>) -> ElementList {
    let mut result = ElementList::new();

    for el in &self.elements[range] {
      let value = el.value.clone();
      let substr = Substr::new(&self.string, el.range);
      let attr_text = AttrText::new(substr, el.markup.clone());
      let res_el = Element::new(value, attr_text);
      result.append(res_el);
    }

    result
  }
}

#[derive(PartialEq, Debug, Clone)]
struct ElbElement {
  value: Value,
  range: SRange,
  markup: Vec<AttrEl>,
}

impl ElbElement {
  fn new(value: Value, range: SRange, markup: Vec<AttrEl>) -> Self {
    ElbElement {
      value,
      range,
      markup,
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_text() {
    let elb = ElementListBuilder::default().add_text("a");
    let expected_el_list =
      ElementList::with_elements(vec![Element::with_substr(Substr::new(&elb.string, 0..1))]);
    assert_eq!(elb.element_list(), expected_el_list);
  }

  #[test]
  fn test_op() {
    let elb = ElementListBuilder::default().add_op("+", "+");
    let expected_el_list = ElementList::with_elements(vec![Element::with_op_in_substr(
      "+",
      Substr::new(&elb.string, 0..1),
    )]);
    assert_eq!(elb.element_list(), expected_el_list);
  }

  #[test]
  fn test_decimal() {
    let elb = ElementListBuilder::default().add_decimal("123", Decimal::new(123, 0));
    let expected_el_list = ElementList::with_elements(vec![Element::with_decimal_in_substr(
      Decimal::new(123, 0),
      Substr::new(&elb.string, 0..3),
    )]);
    assert_eq!(elb.element_list(), expected_el_list);
  }

  #[test]
  fn test_few_elements() {
    let elb = ElementListBuilder::default().add_text("a").add_text("b");
    let expected_el_list = ElementList::with_elements(vec![
      Element::with_substr(Substr::new(&elb.string, 0..1)),
      Element::with_substr(Substr::new(&elb.string, 1..2)),
    ]);
    assert_eq!(elb.element_list(), expected_el_list);
  }

  #[test]
  fn test_integration() {
    let elb = ElementListBuilder::default()
      .add_text("ab")
      .add_int("123", 123)
      .add_op("+", "+");
    let expected_el_list = ElementList::with_elements(vec![
      Element::with_substr(Substr::new(&elb.string, 0..2)),
      Element::with_int_in_substr(123, Substr::new(&elb.string, 2..5)),
      Element::with_op_in_substr("+", Substr::new(&elb.string, 5..6)),
    ]);
    assert_eq!(elb.element_list(), expected_el_list);
  }
}
