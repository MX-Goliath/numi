mod and;
mod any_element;
mod date_matcher;
mod decimal;
mod decimal_unit_sequence;
mod finder;
mod group;
mod not;
mod or;
mod repeat;
mod repr;
mod sequence;
mod text_element;
mod unit;

pub use and::*;
#[allow(unused_imports)]
pub use any_element::*;
pub use date_matcher::*;
pub use decimal::*;
pub use decimal_unit_sequence::*;
pub use finder::*;
pub use group::*;
#[allow(unused_imports)]
pub use not::*;
pub use or::*;
pub use repeat::*;
pub use repr::*;
pub use sequence::*;
#[allow(unused_imports)]
pub use text_element::*;
pub use unit::*;

use crate::ElementList;
use std::{collections::HashMap, ops::Range};

pub type MatcherBox = Box<dyn Matcher>;
pub type MatcherVec = Vec<MatcherBox>;
pub type MatchRange = Range<isize>;
pub type MatchGroups = HashMap<isize, Vec<MatchRange>>;

pub trait Matcher {
  fn matches(&self, element_list: &ElementList, from: isize) -> Vec<Match>;
  fn boxed(self) -> Box<Self>
  where
    Self: Sized,
  {
    Box::new(self)
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Match {
  pub range: MatchRange,
  pub groups: MatchGroups,
}

impl Match {
  pub fn with_range(range: MatchRange) -> Match {
    Match {
      range,
      groups: HashMap::default(),
    }
  }

  pub fn with_element_at(index: isize) -> Match {
    Self::with_range(index..(index + 1))
  }

  pub fn empty_at(index: isize) -> Match {
    Self::with_range(index..index)
  }

  pub fn new(range: MatchRange, groups: MatchGroups) -> Match {
    Match { range, groups }
  }

  pub fn merge(&mut self, other: Match) {
    // potentially better, but need to validate in real-life scenarios:
    // self.range = self.range.start.min(other.range.start)..other.range.end.max(self.range.end);
    self.range = self.range.start..other.range.end;
    for (k, v) in other.groups.iter() {
      let arr = self.groups.entry(*k).or_default();
      arr.extend(v.clone());
    }
  }
}

pub struct TestMatcher {
  matches: Vec<Match>,
}

impl TestMatcher {
  pub fn with_match_range(range: MatchRange) -> Self {
    Self {
      matches: vec![Match::with_range(range)],
    }
  }

  pub fn with_match(m: Match) -> Self {
    Self { matches: vec![m] }
  }

  pub fn new(matches: Vec<Match>) -> Self {
    Self { matches }
  }
}

impl Matcher for TestMatcher {
  fn matches(&self, _element_list: &ElementList, _from: isize) -> Vec<Match> {
    self.matches.clone()
  }
}

pub trait VecMatch {
  fn append_unique_matches(&mut self, ms: Vec<Match>);
  fn append_unique_match(&mut self, m: Match);
}

impl VecMatch for Vec<Match> {
  fn append_unique_matches(&mut self, ms: Vec<Match>) {
    for m in ms {
      self.append_unique_match(m);
    }
  }

  fn append_unique_match(&mut self, m: Match) {
    if !self.contains(&m) {
      self.push(m);
    }
  }
}

#[cfg(test)]
mod matcher_test {

  use super::*;

  #[test]
  fn test_merge() {
    let mut mg1 = MatchGroups::new();
    mg1.insert(1, vec![0..1]);
    let mut mg2 = MatchGroups::new();
    mg2.insert(1, vec![1..2]);
    let mut m1 = Match::new(0..1, mg1);
    let m2 = Match::new(1..2, mg2);
    m1.merge(m2);
    let mut mg_exp = MatchGroups::new();
    mg_exp.insert(1, vec![0..1, 1..2]);
    assert_eq!(m1, Match::new(0..2, mg_exp));
  }
}
