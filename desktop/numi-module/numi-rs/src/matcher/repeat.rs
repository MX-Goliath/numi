use super::MatcherBox;
use crate::ElementList;
use crate::Match;
use crate::Matcher;

pub struct RepeatMatcher {
  matcher: MatcherBox,
  min: Option<isize>,
}

#[derive(PartialEq, Debug, Clone)]
struct RepeatMatch {
  m: Match,
  repeat: isize,
}

impl RepeatMatch {
  fn new(m: Match, repeat: isize) -> Self {
    Self { m, repeat }
  }
}
impl Matcher for RepeatMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    let res = self.matches_rec(element_list, from, 0);
    let mapped = res
      .iter()
      .filter(|&x| {
        if let Some(some_min) = self.min {
          return x.repeat >= some_min;
        }
        true
      })
      .map(|x| x.m.clone())
      .collect();
    mapped
  }
}

impl RepeatMatcher {
  pub fn new(matcher: MatcherBox, min: Option<isize>) -> Self {
    Self { matcher, min }
  }

  fn matches_rec(&self, element_list: &ElementList, from: isize, meet: isize) -> Vec<RepeatMatch> {
    let mut result = Vec::new();

    // Should always return from..from when no minimum (TBD) specified
    // as 'none' repeats is acceptable result, letting other matcher
    // on the same level with repeater to continue from the same spot
    result.push(RepeatMatch::new(Match::with_range(from..from), meet));

    // Return early, esp. important as called recursively
    // Max check should go here
    if from >= element_list.len() {
      return result;
    }

    let matcher_results = self.matcher.matches(element_list, from);

    for m in matcher_results {
      // Matches from passed matcher are not added directly to the results, but only after evaluating
      // "repeats" after them, as all subsequent calls will return "empty" match as a minimum
      let next_matches = self.matches_rec(element_list, m.range.end, meet + 1);
      for next_m in next_matches {
        // Normalize range, by using start from self.matcher's start
        let mut result_match = m.clone();
        result_match.merge(next_m.m);
        result.push(RepeatMatch::new(result_match, next_m.repeat));
      }
    }

    result
  }
}

#[cfg(test)]
mod test {

  use super::*;
  use crate::op::OpMatcher;
  use crate::op::OpType;
  use crate::{
    element_list_builder::ElementListBuilder,
    matcher::{AnyElementMatcher, DecimalMatcher, TextElementMatcher},
  };

  #[test]
  fn test_default() {
    let matcher = RepeatMatcher {
      matcher: OpMatcher::with_op(OpType::Any).boxed(),
      min: None,
    };
    let results = matcher.matches(&ElementList::empty(), 0);
    assert_eq!(results, vec![Match::with_range(0..0)]);
  }

  #[test]
  fn test_single_match() {
    let matcher = RepeatMatcher {
      matcher: AnyElementMatcher::new().boxed(),
      min: None,
    };
    let elb = ElementListBuilder::default().add_text("a");
    let result = matcher.matches(&elb.element_list(), 0);
    assert_eq!(
      result,
      vec![Match::with_range(0..0), Match::with_range(0..1)]
    );
  }

  #[test]
  fn test_element_repeat_till_end() {
    let matcher = RepeatMatcher {
      matcher: AnyElementMatcher::new().boxed(),
      min: None,
    };
    let elb = ElementListBuilder::default().add_text("a").add_text("b");
    let result = matcher.matches(&elb.element_list(), 0);
    assert_eq!(
      result,
      vec![
        Match::with_range(0..0),
        Match::with_range(0..1),
        Match::with_range(0..2)
      ]
    );
  }

  #[test]
  fn test_element_repeat_till_no_match() {
    let matcher = RepeatMatcher {
      matcher: TextElementMatcher::new().boxed(),
      min: None,
    };
    let elb = ElementListBuilder::default().add_text("a").add_int("1", 1);
    let result = matcher.matches(&elb.element_list(), 0);
    assert_eq!(
      result,
      vec![Match::with_range(0..0), Match::with_range(0..1)]
    );
  }

  #[test]
  fn test_min() {
    let matcher = RepeatMatcher {
      matcher: AnyElementMatcher::new().boxed(),
      min: Some(2),
    };
    let elb = ElementListBuilder::default().add_text("a").add_text("b");
    let result = matcher.matches(&elb.element_list(), 0);
    assert_eq!(result, vec![Match::with_range(0..2),]);
  }

  #[test]
  fn test_zero_min() {
    let matcher = RepeatMatcher {
      matcher: DecimalMatcher::new().boxed(),
      min: None,
    };
    let elb = ElementListBuilder::default().add_text("a");
    let result = matcher.matches(&elb.element_list(), 0);
    assert_eq!(result, vec![Match::with_range(0..0),]);
  }
}
