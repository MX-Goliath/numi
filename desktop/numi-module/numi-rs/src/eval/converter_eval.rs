use self::tz::TzMatcher;
use crate::matcher::*;
use crate::unit_converter::UnitConverter;
use crate::*;
use op::{OpMatcher, OpType};
use std::cell::RefCell;
use std::rc::Rc;

pub struct ConverterEval {
  matcher: ConversionMatcher,
  unit_converter: Rc<RefCell<UnitConverter>>,
}

impl ConverterEval {
  pub fn new(unit_converter: Rc<RefCell<UnitConverter>>) -> Self {
    Self {
      matcher: ConversionMatcher::new(),
      unit_converter,
    }
  }
}

impl Evaluator for ConverterEval {
  fn evaluate<'a>(
    &self,
    elements: &ElementList<'a>,
    context: &EvalContext,
  ) -> Option<ElementList<'a>> {
    let matches = self.matcher.matches(elements, 0);
    let m = matches.first()?;

    let value = elements.get(m.range.start).unwrap().value.clone();
    let to_unit_value = &elements.get(m.range.start + 2).unwrap().value;

    // dbg!(&value);
    // dbg!(&to_unit_value);

    let mut result = elements.clone();
    if let Some(result_value) =
      self
        .unit_converter
        .borrow()
        .convert(&value, to_unit_value, &context.data)
    {
      result.replace_elements_with_value(m.range.clone(), result_value);
      return result.some();
    }
    result.replace_elements_with_value(0..elements.len(), Value::with_error());
    Some(result)
  }
}

#[cfg(test)]
mod decimal_converter_test {
  use rust_decimal_macros::dec;

  use crate::unit_converter::RatioValueConverter;

  use super::*;

  #[test]
  fn test_decimal_without_unit_conversion() {
    let eval = ConverterEval::new(Rc::new(RefCell::new(UnitConverter::default())));
    let list_b = ElementList::builder()
      .add_int("1", 1)
      .add_op(" in ", "in")
      .add_unit("m", "m");
    let result = eval
      .evaluate(&list_b.element_list(), &EvalContext::dumb())
      .unwrap();
    let expected_value = Value::default().with_int(1).with_unit("m");
    assert_eq!(
      result,
      Element::new(expected_value, list_b.element_list().attr_text().unwrap()).into_list()
    );
  }

  #[test]
  fn test_conversion() {
    let mut ratio_conv = RatioValueConverter::default();
    ratio_conv.set_ratio("cm", "m", dec!(0.01));
    let mut converter = UnitConverter::default();
    converter.add_converter(Box::new(ratio_conv));
    let eval = ConverterEval::new(Rc::new(RefCell::new(converter)));

    let list_b = ElementList::builder()
      .add_int_unit("1", 1, "m")
      .add_op(" in ", "in")
      .add_unit("cm", "cm");
    let result = eval
      .evaluate(&list_b.element_list(), &EvalContext::dumb())
      .unwrap();
    let expected_value = Value::default().with_int(100).with_unit("cm");
    assert_eq!(
      result,
      Element::new(expected_value, list_b.element_list().attr_text().unwrap()).into_list()
    );
  }
}

pub struct ConversionMatcher {
  matcher: MatcherBox,
}

impl ConversionMatcher {
  pub fn new() -> Self {
    Self {
      matcher: SequenceMatcher::with_matchers(vec![
        OrMatcher::with_matchers(vec![
          DecimalMatcher::new().boxed(),
          DateMatcher::new().boxed(),
        ])
        .boxed(),
        OpMatcher::with_op(OpType::Some("in".to_string())).boxed(),
        OrMatcher::with_matchers(vec![
          UnitMatcher::default().boxed(),
          TzMatcher::default().boxed(),
        ])
        .boxed(),
      ])
      .boxed(),
    }
  }
}

impl Matcher for ConversionMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    self.matcher.matches(element_list, from)
  }
}

#[cfg(test)]
mod decimal_conversion_matcher_test {

  use super::*;
  use crate::element_list_builder::ElementListBuilder;

  #[test]
  fn test() {
    let matcher = ConversionMatcher::new();
    let elb = ElementListBuilder::default()
      .add_int("1", 1)
      .add_op("in", "in")
      .add_unit("m", "m");
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..3)]);
  }
}
