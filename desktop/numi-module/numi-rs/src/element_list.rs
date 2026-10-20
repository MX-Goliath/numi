use crate::element_list_builder::ElementListBuilder;

use super::*;
use core::slice::Iter;
use rust_decimal::Decimal;
use std::cmp::{max, min};
use std::ops::Range;
use std::vec::IntoIter;

#[derive(PartialEq, Debug, Clone)]
pub struct ElementList<'a> {
  pub elements: Vec<Element<'a>>,
}

impl<'a> ElementList<'a> {
  pub fn new() -> ElementList<'a> {
    ElementList {
      elements: Vec::new(),
    }
  }

  pub fn empty() -> ElementList<'a> {
    Self::new()
  }

  pub fn builder() -> ElementListBuilder {
    ElementListBuilder::default()
  }

  pub fn some(self) -> Option<Self> {
    Some(self)
  }

  // Helper constructors

  pub fn with_substr(substr: Substr<'a>) -> ElementList<'a> {
    ElementList::with_element(Element::with_substr(substr))
  }

  pub fn with_str(str: &'a str) -> ElementList<'a> {
    ElementList::with_element(Element::with_str(str))
  }

  pub fn with_decimal(decimal: Decimal, str: &str) -> ElementList {
    ElementList::with_element(Element::with_decimal_in_str(decimal, str))
  }

  pub fn with_element(element: Element) -> ElementList {
    ElementList::with_elements(vec![element])
  }

  pub fn with_elements(elements: Vec<Element>) -> ElementList {
    let mut result = ElementList::new();
    for element in elements {
      result.append(element);
    }
    result
  }

  pub fn element_list_to(&self, to: isize) -> ElementList<'a> {
    self.element_list_in(0..max(to, 0))
  }

  pub fn element_list_from(&self, from: isize) -> ElementList<'a> {
    self.element_list_in(min(from, self.len())..self.len())
  }

  pub fn element_list_in(&self, range: Range<isize>) -> ElementList<'a> {
    let mut result = ElementList::new();
    for i in range {
      let i_element = &self.elements[i as usize];
      result.append(i_element.clone());
    }
    result
  }

  // Accessing elements

  pub fn first(&self) -> Option<&Element<'a>> {
    self.get(0)
  }

  pub fn last(&self) -> Option<&Element<'a>> {
    if !self.is_empty() {
      return self.get(self.len() - 1);
    }
    None
  }

  pub fn get(&self, index: isize) -> Option<&Element<'a>> {
    self.elements.get(index as usize)
  }

  // Modify elements

  pub fn append_list(&mut self, element_list: ElementList<'a>) {
    for el in element_list.into_iter() {
      self.append(el);
    }
  }

  pub fn append(&mut self, element: Element<'a>) {
    self.insert(element, self.len());
  }

  pub fn insert(&mut self, element: Element<'a>, at_index: isize) {
    if cfg!(debug_assertions) {
      for el in &self.elements {
        debug_assert!(
          el.attr_text.can_merge(&element.attr_text),
          "Attributed text can't be merged into current element list"
        );
      }
    }
    self.elements.insert(at_index as usize, element);
  }

  pub fn iter(&self) -> Iter<Element<'a>> {
    self.elements.iter()
  }

  pub fn into_iter(self) -> IntoIter<Element<'a>> {
    self.elements.into_iter()
  }

  pub fn whole_str(&self) -> Option<&'a str> {
    if let Some(el) = self.first() {
      return Some(el.attr_text.whole_str());
    }
    None
  }

  pub fn is_empty(&self) -> bool {
    self.elements.is_empty()
  }

  pub fn len(&self) -> isize {
    self.elements.len() as isize
  }

  pub fn attr_text(&self) -> Option<AttrText<'a>> {
    self.attr_text_in_range(0..self.len())
  }

  pub fn attr_text_in_range(&self, range: Range<isize>) -> Option<AttrText<'a>> {
    debug_assert!(
      range.start < self.len() && range.end <= self.len(),
      "Range out of bounds"
    );
    if let Some(first_el) = self.get(range.start) {
      let first_str = first_el.attr_text.whole_str();
      let substr = Substr::with_empty_str(first_str);
      let mut acc_attr_text = AttrText::with_substr(substr);

      // todo: optimize
      for i in range {
        if let Some(i_el) = self.get(i) {
          acc_attr_text.merge_serial(i_el.attr_text.clone())
        }
      }
      return Some(acc_attr_text);
    }
    None
  }

  pub fn has_error(&self) -> bool {
    for el in self.elements.iter() {
      if el.value.has_error() {
        return true;
      }
    }
    false
  }

  pub fn replace_element(&mut self, at: isize, to: Element<'a>) {
    self.replace_elements(at..at + 1, ElementList::with_element(to));
  }

  pub fn replace_elements(&mut self, range: Range<isize>, replace_to: ElementList<'a>) {
    let mut insert_at = range.start;
    for i in range.rev() {
      self.elements.remove(i as usize);
    }
    for el in replace_to.elements {
      self.insert(el, insert_at);
      insert_at += 1;
    }
  }

  pub fn replace_elements_with_value(&mut self, range: Range<isize>, value: Value) {
    let attr_text = self.attr_text_in_range(range.clone());
    let el = Element::new(value, attr_text.unwrap());
    self.replace_elements(range, ElementList::with_element(el));
  }

  pub fn replace_elements_with_attr_value(
    &mut self,
    range: Range<isize>,
    kind: AttrKind,
    value: Value,
  ) {
    let attr_text = self
      .attr_text_in_range(range.clone())
      .unwrap()
      .of_kind(kind);
    let el = Element::new(value, attr_text);
    self.replace_elements(range, ElementList::with_element(el));
  }
}

impl<'a> Default for ElementList<'a> {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {

  use crate::{element_list_builder::ElementListBuilder, Value};
  use pretty_assertions::assert_eq;

  #[test]
  fn test_replace() {
    let elb = ElementListBuilder::default().add_text("a").add_text("1");
    let mut list = elb.element_list();
    list.replace_element(1, elb.int_at(1, 1));
    let expected_elb = ElementListBuilder::default().add_text("a").add_int("1", 1);
    assert_eq!(list, expected_elb.element_list());
  }

  #[test]
  fn combined_serial_attr_text_in_range() {}

  #[test]
  fn combined_serial_attr_text() {}

  #[test]
  fn test_element_list_in() {
    let elb = ElementListBuilder::default().add_text("a").add_text("b");
    let el_list = elb.element_list();
    let result = el_list.element_list_in(1..2);
    let expected = elb.element_list_in_range(1..2);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_add_list() {
    let elb = ElementListBuilder::default()
      .add_text("a")
      .add_text("b")
      .add_text("c");
    let mut result = elb.element_list_in_range(0..1);
    result.append_list(elb.element_list_in_range(1..2));
    assert_eq!(result, elb.element_list_in_range(0..2));
  }

  #[test]
  fn test_replace_elements_in_range_with_value() {
    let elb = ElementListBuilder::default().add_text("a").add_int("1", 1);
    let mut list = elb.element_list();
    list.replace_elements_with_value(0..2, Value::default().with_int(2));
    assert_eq!(list.len(), 1);
    assert_eq!(list.attr_text(), elb.element_list().attr_text());
    assert_eq!(list.get(0).unwrap().value, Value::default().with_int(2));
  }
}
