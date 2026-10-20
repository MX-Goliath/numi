use crate::{
  matcher::{DecimalMatcher, FinderMatcher, Match, Matcher, RepeatMatcher, SequenceMatcher},
  ElementList,
};

pub struct FuncExpressionMatcher {
  matcher: FinderMatcher,
}

impl Default for FuncExpressionMatcher {
  fn default() -> Self {
    Self {
      matcher: FinderMatcher::with_matcher(
        SequenceMatcher::with_matchers(vec![
          FuncMatcher::default().boxed(),
          RepeatMatcher::new(DecimalMatcher::new().boxed(), None).boxed(),
        ])
        .boxed(),
      ),
    }
  }
}

impl Matcher for FuncExpressionMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    self.matcher.matches(element_list, from)
  }
}

#[derive(Default)]
pub struct FuncMatcher {}

impl Matcher for FuncMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    if let Some(el) = element_list.get(from) {
      if el.value.has_func() {
        return vec![Match::with_element_at(from)];
      }
    }
    Vec::new()
  }
}

#[cfg(test)]
mod func_expression_matcher_test {
  use super::*;
  use crate::element_list_builder::ElementListBuilder;

  #[test]
  fn test() {
    let matcher = FuncExpressionMatcher::default();
    let elb = ElementListBuilder::default()
      .add_func("a", "a")
      .add_int("1", 1);
    let matches = matcher.matches(&elb.element_list(), 0);
    assert_eq!(matches, vec![Match::with_range(0..2)]);
  }
}
