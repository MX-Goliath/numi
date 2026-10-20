use crate::ElementList;
use crate::Match;
use crate::Matcher;

use super::MatcherVec;

pub struct SequenceMatcher {
  matchers: MatcherVec,
}

impl SequenceMatcher {
  pub fn with_matchers(matchers: MatcherVec) -> SequenceMatcher {
    SequenceMatcher { matchers }
  }
}

impl Matcher for SequenceMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    self.matches_for_matcher_at(element_list, from, 0)
  }
}

impl SequenceMatcher {
  fn matches_for_matcher_at(
    &self,
    element_list: &ElementList,
    from: isize,
    matcher_index: usize,
  ) -> Vec<Match> {
    // Only here "result" is actually populated, as in other places
    // it actually modified by calling `matches_for_matcher_at` recursively
    if matcher_index >= self.matchers.len() {
      return vec![Match::empty_at(from)];
    }

    let mut result = Vec::new();

    if let Some(matcher) = self.matchers.get(matcher_index) {
      // Find all matches for matcher at index `matcher_index`
      let matches = matcher.matches(element_list, from);
      for m in matches {
        // Find next matches, the result will be populated only when matcher_index reached
        // the end (see above) and later will be normalized with m.start
        let next_matches =
          self.matches_for_matcher_at(element_list, m.range.end, matcher_index + 1);
        for next_m in next_matches {
          let mut result_match = m.clone();
          result_match.merge(next_m);
          result.push(result_match);
        }
      }
    }

    result
  }
}
