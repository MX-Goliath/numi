use crate::ElementList;
use crate::Match;
use crate::Matcher;

#[derive(Default)]
pub struct TzMatcher {}

impl Matcher for TzMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    if let Some(el) = element_list.get(from) {
      if el.value.is_timezone() {
        return vec![Match::with_element_at(from)];
      }
    }
    Vec::new()
  }
}
