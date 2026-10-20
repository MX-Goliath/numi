use std::collections::HashMap;

use crate::{
  phrase_detector::{PhraseDetector, PhraseMatch},
  substr::Substr,
};

pub type UnitId = String;

#[derive(Default, Clone)]
pub struct UnitManager {
  units: HashMap<UnitId, Unit>,
  detector: PhraseDetector,
}

impl UnitManager {
  pub fn add_unit(&mut self, unit: Unit, phrases: &Vec<&str>) {
    self.detector.add_normalized_phrases_for(&unit.id, phrases);
    self.units.insert(unit.id.clone(), unit);
  }

  pub fn add_phrases(&mut self, unit_id: &str, phrases: &Vec<&str>) {
    self.detector.add_phrases_for(unit_id, phrases);
  }

  pub fn format(&self, unit_id: &str) -> Option<&str> {
    if let Some(unit) = self.units.get(unit_id) {
      return Some(unit.format.as_str());
    }
    None
  }

  pub fn matches<'a, 's, S>(&'a self, into_substr: S) -> Vec<PhraseMatch<'a, 's>>
  where
    S: Into<Substr<'s>>,
  {
    self.detector.matches(into_substr)
  }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Unit {
  pub id: UnitId,
  pub format: String,
}

impl Unit {
  pub fn new(id: &str, format: &str) -> Self {
    Self {
      id: id.to_string(),
      format: format.to_string(),
    }
  }
}
