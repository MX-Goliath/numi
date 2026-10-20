use super::TzMatcher;
use crate::{
  matcher::{DateMatcher, FinderMatcher, MatcherBox, SequenceMatcher},
  Match, Matcher,
};

pub struct TzDateMatcher {
  matcher: MatcherBox,
}

impl TzDateMatcher {
  pub fn new() -> Self {
    let sequence = SequenceMatcher::with_matchers(vec![
      Box::<TzMatcher>::default(),
      Box::new(DateMatcher::new()),
    ]);
    let m = FinderMatcher::with_matcher(sequence.boxed());
    Self { matcher: m.boxed() }
  }
}

impl Matcher for TzDateMatcher {
  fn matches(&self, element_list: &crate::element_list::ElementList, from: isize) -> Vec<Match> {
    self.matcher.matches(element_list, from)
  }
}

#[cfg(test)]
mod tz_date_matcher_test {

  use super::*;
  use crate::{date::DateTime, element_list_builder::ElementListBuilder, matcher::Match};

  #[test]
  fn test() {
    let matcher = TzDateMatcher::new();
    let elb = ElementListBuilder::default()
      .add_timezone("UTC", "UTC")
      .add_date(
        "2021-01-01",
        DateTime::with_str_datetime("2021-01-01 00:00:00").unwrap(),
      );
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..2)]);
  }
}
