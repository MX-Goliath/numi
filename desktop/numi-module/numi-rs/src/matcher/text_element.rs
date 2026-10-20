use crate::ElementList;
use crate::Match;
use crate::Matcher;

pub struct TextElementMatcher {}

impl TextElementMatcher {
  pub fn new() -> Self {
    Self {}
  }
}

impl Matcher for TextElementMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    if from < element_list.len() {
      if let Some(el) = element_list.get(from) {
        if !el.has_some_value() {
          return vec![Match::with_element_at(from)];
        }
      }
    }
    Vec::new()
  }
}

#[cfg(test)]
mod test {

  use rust_decimal_macros::dec;

  use super::*;

  #[test]
  fn test() {
    let matcher = TextElementMatcher::new();
    let str = "a";
    let matches = matcher.matches(&ElementList::with_str(str), 0);
    assert_eq!(matches, vec![Match::with_range(0..1)]);
  }

  #[test]
  fn test_no_match() {
    let matcher = TextElementMatcher::new();
    let str = "a";
    let matches = matcher.matches(&ElementList::with_decimal(dec!(1), str), 0);
    assert_eq!(matches, vec![]);
  }
}
