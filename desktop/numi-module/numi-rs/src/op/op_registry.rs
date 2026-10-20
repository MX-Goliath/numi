use super::Op;
use crate::op::OpType;
use crate::{
  phrase_detector::{PhraseDetector, PhraseMatch},
  str_ranger::StrRanger,
  substr::Substr,
};
use std::collections::HashMap;

pub struct OpRegistry {
  detector: PhraseDetector,
  binary_op_map: HashMap<OpType, isize>,
}

impl Default for OpRegistry {
  fn default() -> Self {
    Self {
      detector: PhraseDetector::new(StrRanger::new(), true),
      binary_op_map: HashMap::new(),
    }
  }
}

impl OpRegistry {
  pub fn add_op(&mut self, op: Op, weight: Option<isize>, phrases: Vec<&str>) {
    if let Some(weight) = weight {
      self
        .binary_op_map
        .insert(OpType::Some(op.id.clone()), weight);
    }
    for phrase in phrases {
      self.detector.add_phrase_for(&op.id, phrase);
    }
  }

  pub fn binary_op_map(&self) -> HashMap<OpType, isize> {
    self.binary_op_map.clone()
  }

  pub fn matches<'a, 's, S>(&'a self, into_substr: S) -> Vec<PhraseMatch<'a, 's>>
  where
    S: Into<Substr<'s>>,
  {
    self.detector.matches(into_substr)
  }
}
