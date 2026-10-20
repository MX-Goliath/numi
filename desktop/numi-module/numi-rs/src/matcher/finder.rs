use crate::ElementList;
use crate::Match;
use crate::Matcher;

/// The logic of `FinderMatcher` is slightly sophisticated:
/// It finds the first matches from some index, takes just one match with max range
/// and tries to find next sequence from the end of the previous max match
pub struct FinderMatcher {
  matcher: Box<dyn Matcher>,
}

// FinderAllMatcher ?
impl FinderMatcher {
  pub fn with_matcher(matcher: Box<dyn Matcher>) -> FinderMatcher {
    FinderMatcher { matcher }
  }
}

impl Matcher for FinderMatcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match> {
    let mut result = Vec::new();

    let mut index = from;

    while element_list.get(index).is_some() {
      let match_results = self.matcher.matches(element_list, index);
      let max_match = match_results
        .iter()
        .max_by(|a, b| a.range.end.partial_cmp(&b.range.end).unwrap());
      if let Some(max_match) = max_match {
        result.push(max_match.clone());
        index = max_match.range.end - 1 // -1 to compensate next +1
      }
      index += 1;
    }

    result
  }
}
