use crate::ElementList;
use crate::Match;
use crate::Matcher;

pub struct DecimalMatcher {}

impl DecimalMatcher {
  pub fn new() -> DecimalMatcher {
    DecimalMatcher {}
  }
}

impl Matcher for DecimalMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    if let Some(el) = element_list.get(from) {
      if el.value.is_number() {
        return vec![Match::with_element_at(from)];
      }
    }
    Vec::new()
  }
}
