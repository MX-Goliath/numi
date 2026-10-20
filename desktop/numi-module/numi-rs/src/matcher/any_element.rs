use crate::ElementList;
use crate::Match;
use crate::Matcher;

pub struct AnyElementMatcher {}

impl AnyElementMatcher {
  pub fn new() -> AnyElementMatcher {
    AnyElementMatcher {}
  }
}

impl Matcher for AnyElementMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    if from < element_list.len() {
      return vec![Match::with_element_at(from)];
    }
    Vec::new()
  }
}
