use crate::{
  attr_text::{AttrKind, AttrText},
  element::Element,
  element_list::ElementList,
  eval::Evaluator,
  eval_context::EvalContext,
  phrase_detector::PhraseDetector,
};

#[derive(Default)]
pub struct VarExtractor {}

impl Evaluator for VarExtractor {
  fn evaluate<'el>(
    &self,
    elements: &ElementList<'el>,
    context: &EvalContext,
  ) -> Option<ElementList<'el>> {
    if let Some(first_el) = elements.first() {
      let substr = first_el.attr_text.substr.clone();
      let mut pd = PhraseDetector::default();
      pd.register_phrases(
        &context
          .data
          .all_variables()
          .iter()
          .map(|v| v.as_str())
          .collect(),
      );
      let matches = pd.matches(substr);
      let res_elements: Vec<Element> = matches
        .into_iter()
        .map(|m| {
          return Element::new(
            context.data.get_variable(m.id),
            AttrText::with_substr_of_kind(m.substr, AttrKind::Var),
          );
        })
        .collect();
      ElementList::with_elements(res_elements).some()
    } else {
      None
    }
  }
}

#[cfg(test)]
mod test {
  use crate::{substr::Substr, Value};

  use super::*;

  #[test]
  fn test_var_extractor() {
    let eval_context = EvalContext::dumb();
    eval_context
      .data
      .set_variable("b", &Value::default().with_int(1));
    let evaluator = VarExtractor::default();
    let str = "a b c";
    let input_el_list = ElementList::with_str(str);
    let result = evaluator.evaluate(&input_el_list, &eval_context);
    let el = Element::new(
      Value::default().with_int(1),
      AttrText::with_substr_of_kind(Substr::new(str, 2..3), AttrKind::Var),
    );
    assert_eq!(result.unwrap(), ElementList::with_element(el));
  }
}
