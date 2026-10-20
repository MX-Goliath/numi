use super::Evaluator;
use crate::matcher::{DecimalUnitSequenceMatcher, Matcher};
use crate::op::BinaryOp;
use crate::op::DecimalOp;
use crate::unit_converter::UnitConverter;
use std::{cell::RefCell, rc::Rc};

pub struct DecUnitSequenceEval {
  matcher: DecimalUnitSequenceMatcher,
  op_calc: DecimalOp,
}

impl DecUnitSequenceEval {
  pub fn new(converter: Rc<RefCell<UnitConverter>>) -> Self {
    Self {
      matcher: DecimalUnitSequenceMatcher::new(),
      op_calc: DecimalOp { converter },
    }
  }
}

impl Evaluator for DecUnitSequenceEval {
  fn evaluate<'a>(
    &self,
    elements: &crate::element_list::ElementList<'a>,
    context: &crate::eval_context::EvalContext,
  ) -> Option<crate::element_list::ElementList<'a>> {
    let matches = self.matcher.matches(elements, 0);
    let m = matches.first()?;
    let gm = m.groups.get(&1)?;
    let first_range = gm.first()?;
    let mut result_v = elements.get(first_range.start).unwrap().value.clone();
    for r in gm.iter().skip(1) {
      let v = &elements.get(r.start).unwrap().value;
      let sum_v = self.op_calc.evaluate_op("+", &result_v, v, &context.data)?;
      result_v = sum_v;
    }
    let mut result = elements.clone();
    result.replace_elements_with_value(m.range.clone(), result_v);
    result.some()
  }
}

#[cfg(test)]
mod dec_unit_seq_test {
  use super::*;
  use crate::{
    element::Element,
    element_list::ElementList,
    eval::DecUnitSequenceEval,
    eval_context::EvalContext,
    unit_converter::{RatioValueConverter, UnitConverter},
    Value,
  };
  use rust_decimal_macros::dec;
  use std::{cell::RefCell, rc::Rc};

  #[test]
  fn test() {
    let mut ratio_conv = RatioValueConverter::default();
    ratio_conv.set_ratio("cm", "m", dec!(0.01));
    let mut converter = UnitConverter::default();
    converter.add_converter(Box::new(ratio_conv));
    let eval = DecUnitSequenceEval::new(Rc::new(RefCell::new(converter)));

    let list_b = ElementList::builder()
      .add_int_unit("1m ", 1, "m")
      .add_int_unit("5cm", 5, "cm");
    let result = eval
      .evaluate(&list_b.element_list(), &EvalContext::dumb())
      .unwrap();
    let expected_value = Value::default().with_int(105).with_unit("cm");
    assert_eq!(
      result,
      Element::new(expected_value, list_b.element_list().attr_text().unwrap()).into_list()
    );
  }
}
