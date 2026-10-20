use super::{
  AndMatcher, DecimalMatcher, FinderMatcher, GroupMatcher, Matcher, MatcherBox, RepeatMatcher,
  UnitMatcher,
};

pub struct DecimalUnitSequenceMatcher {
  matcher: MatcherBox,
}

impl DecimalUnitSequenceMatcher {
  pub fn new() -> Self {
    let decimal_unit_m = AndMatcher::with_matchers(vec![
      Box::<UnitMatcher>::default(),
      Box::new(DecimalMatcher::new()),
    ]);
    let group = GroupMatcher::new(1, decimal_unit_m.boxed());
    let repeat_m = RepeatMatcher::new(group.boxed(), Option::Some(2));
    let m = FinderMatcher::with_matcher(repeat_m.boxed());
    Self { matcher: m.boxed() }
  }
}

impl Matcher for DecimalUnitSequenceMatcher {
  fn matches(
    &self,
    element_list: &crate::element_list::ElementList,
    from: isize,
  ) -> Vec<super::Match> {
    self.matcher.matches(element_list, from)
  }
}

#[cfg(test)]
mod decimal_seq_test {

  use crate::{
    element_list_builder::ElementListBuilder,
    matcher::{Match, MatchGroups},
  };

  use super::*;

  #[test]
  fn test() {
    let matcher = DecimalUnitSequenceMatcher::new();
    let elb = ElementListBuilder::default()
      .add_int_unit("1 m", 1, "m")
      .add_int_unit("4 cm", 4, "cm");
    let matches = matcher.matches(&elb.element_list(), 0);
    let mut mg = MatchGroups::new();
    mg.insert(1, vec![0..1, 1..2]);
    assert_eq!(matches, vec![Match::new(0..2, mg)]);
  }
}
