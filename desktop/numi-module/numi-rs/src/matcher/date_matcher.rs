use crate::ElementList;
use crate::Match;
use crate::Matcher;

pub struct DateMatcher {}

impl DateMatcher {
  pub fn new() -> DateMatcher {
    DateMatcher {}
  }
}

impl Matcher for DateMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    if let Some(el) = element_list.get(from) {
      if el.value.is_datetime() {
        return vec![Match::with_element_at(from)];
      }
    }
    Vec::new()
  }
}
