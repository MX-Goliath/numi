use std::cmp::max;
use std::cmp::min;
use std::fmt::*;
use std::ops::Range;

/// Range type, alternative to `Range<usize>` to operate on strings
/// in more convenient manner.
/// Derives from `Copy` and provides some helper methods for
/// bounds checks and merge
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct SRange {
  pub start: usize,
  pub len: usize,
}

impl SRange {
  // - init

  pub fn empty() -> SRange {
    SRange::new(0, 0)
  }

  pub fn from_range(range: Range<usize>) -> SRange {
    SRange {
      start: range.start,
      len: range.end - range.start,
    }
  }

  pub fn new(start: usize, len: usize) -> SRange {
    SRange { start, len }
  }

  // - merge and split

  /// Merges with other range by using min start and max end
  /// from either of ranges
  pub fn merge(&mut self, other: SRange) -> Self {
    if other.is_empty() {
      return *self;
    }
    if self.is_empty() {
      self.start = other.start;
      self.len = other.len;
      return *self;
    }
    let end = max(self.end(), other.end());
    let start = min(self.start, other.start);
    self.start = start;
    self.len = end - start;
    *self
  }

  /// Assert that range is located right before or after (debug only)
  /// Otherwise merges with `merge`
  pub fn merge_serial(&mut self, other: SRange) -> Self {
    debug_assert!(
      self.end() == other.start || other.end() == self.start || self.is_empty() || other.is_empty(),
      "Merge serial require ranges to go one after another: {self:?}, {other:?}",
    );
    self.merge(other)
  }

  /// Assert ranges are not intersected (debug only)
  /// Otherwise merges with `merge`
  pub fn merge_disjoint(&mut self, other: SRange) -> Self {
    debug_assert!(
      !self.is_intersect(&other),
      "Disjoint merge has interesctions: {self:?}, {other:?}",
    );
    self.merge(other)
  }

  pub fn split_by_range(&self, other: &SRange) -> Vec<SRange> {
    let mut result = vec![];

    if self.is_intersect(other) {
      // sort ranges by first, second
      let mut first = self;
      let mut second = other;
      if first.start > second.start {
        first = other;
        second = self;
      }

      // first subrange always included
      result.push(SRange::new(first.start, second.start - first.start));

      // if second range contained within first one completely
      if first.has(second.start) && first.has(second.end()) {
        // add second and what is left from the end
        result.push(*second);
        result.push(SRange::new(second.end(), first.end() - second.end()));
      } else {
        // otherwise second range ends after first range
        result.push(SRange::new(second.start, first.end() - second.start));
        result.push(SRange::new(first.end(), second.end() - first.end()));
      }
    } else {
      // if no intersection, add self and other
      result.push(*self);
      result.push(*other);
    }
    // filter-out empty ranges
    let filtered_result: Vec<SRange> = result.into_iter().filter(|range| range.len > 0).collect();
    filtered_result
  }

  pub fn split_by_location(&self, location: usize) -> Vec<SRange> {
    let mut result = vec![];
    if self.has(location) {
      let r1 = SRange::new(self.start, location - self.start);
      if r1.len > 0 {
        result.push(r1);
      }
      let r2 = SRange::new(location, self.end() - location);
      if r2.len > 0 {
        result.push(r2);
      }
    } else {
      result.push(*self);
    }
    result
  }

  // - other

  pub fn to_range(self) -> Range<usize> {
    self.start..self.end()
  }

  /// Calculate index next to the range end (start + len)
  pub fn end(&self) -> usize {
    self.start + self.len
  }

  pub fn is_in(&self, range: SRange) -> bool {
    if self.is_empty() || range.is_empty() {
      return false;
    }
    self.start >= range.start && self.end() <= range.end()
  }

  pub fn is_intersect(&self, range: &SRange) -> bool {
    if self.is_empty() || range.is_empty() {
      return false;
    }
    self.has(range.start)
      || self.has(range.end() - 1)
      || range.has(self.start)
      || range.has(self.end() - 1)
  }

  pub fn has(&self, location: usize) -> bool {
    self.start <= location && self.end() > location
  }

  pub fn is_empty(&self) -> bool {
    self.len == 0
  }
}

impl Default for SRange {
  fn default() -> Self {
    SRange::empty()
  }
}

impl From<Range<usize>> for SRange {
  fn from(range: Range<usize>) -> Self {
    Self::from_range(range)
  }
}

/// Extends `Range<usize>` with some helper methods
/// Only possible with helper traits such this
pub trait RangeUsizeExt {
  fn to_srange(self) -> SRange;
}

impl RangeUsizeExt for Range<usize> {
  fn to_srange(self) -> SRange {
    SRange::from_range(self)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_from_range() {
    assert_eq!(SRange::new(0, 0), SRange::from_range(0..0));
    assert_eq!(SRange::new(0, 1), SRange::from_range(0..1));
    assert_eq!(SRange::new(1, 2), SRange::from_range(1..3));
  }

  #[test]
  fn test_is_in() {
    assert!(SRange::new(0, 1).is_in(SRange::new(0, 1)));
    assert!(SRange::new(1, 1).is_in(SRange::new(0, 3)));
    assert!(!SRange::new(1, 1).is_in(SRange::new(0, 1)));
    assert!(!SRange::new(0, 1).is_in(SRange::new(1, 1)));
  }

  #[test]
  fn test_is_intersect() {
    assert!(SRange::new(0, 1).is_intersect(&SRange::new(0, 1)));
    assert!(SRange::new(0, 1).is_intersect(&SRange::new(0, 2)));
    assert!(SRange::new(0, 2).is_intersect(&SRange::new(0, 1)));
    assert!(SRange::new(0, 2).is_intersect(&SRange::new(1, 2)));
    assert!(SRange::new(0, 3).is_intersect(&SRange::new(1, 1)));
    assert!(SRange::new(1, 1).is_intersect(&SRange::new(0, 3)));
    assert!(!SRange::new(0, 1).is_intersect(&SRange::new(1, 1)));
  }

  // - merge

  #[test]
  fn test_merge() {
    assert_eq!(
      SRange::new(0, 1).merge(SRange::new(1, 1)),
      SRange::new(0, 2)
    );
    assert_eq!(
      SRange::new(0, 1).merge(SRange::new(3, 0)),
      SRange::new(0, 1)
    );
    assert_eq!(
      SRange::new(2, 0).merge(SRange::new(0, 1)),
      SRange::new(0, 1)
    );
    assert_eq!(
      SRange::new(0, 1).merge(SRange::new(2, 1)),
      SRange::new(0, 3)
    );
  }

  #[test]
  fn test_serial() {
    assert_eq!(
      SRange::new(0, 1).merge_serial(SRange::new(1, 1)),
      SRange::new(0, 2)
    );
    assert_eq!(
      SRange::new(0, 1).merge_serial(SRange::new(2, 0)),
      SRange::new(0, 1)
    );
  }

  #[test]
  #[should_panic]
  fn test_merge_serial_panic() {
    SRange::new(0, 1).merge_serial(SRange::new(2, 1));
  }

  #[test]
  fn test_merge_disjoint() {
    assert_eq!(
      SRange::new(0, 1).merge_disjoint(SRange::new(1, 1)),
      SRange::new(0, 2)
    );
    assert_eq!(
      SRange::new(0, 1).merge_disjoint(SRange::new(2, 1)),
      SRange::new(0, 3)
    );
  }

  #[test]
  #[should_panic]
  fn test_merge_disjoint_panic() {
    SRange::new(0, 2).merge_disjoint(SRange::new(1, 1));
  }

  // split

  #[test]
  fn test_split_location_no_intersect() {
    let s1 = SRange::new(0, 1);
    assert_eq!(s1.split_by_location(2), vec![s1]);
  }

  #[test]
  fn test_split_location() {
    let s1 = SRange::new(0, 2);
    assert_eq!(
      s1.split_by_location(1),
      vec![SRange::new(0, 1), SRange::new(1, 1)]
    );
  }

  #[test]
  fn test_split_location_at_start() {
    let s1 = SRange::new(0, 1);
    assert_eq!(s1.split_by_location(0), vec![s1]);
  }

  #[test]
  fn test_split_location_at_end() {
    let s1 = SRange::new(0, 1);
    assert_eq!(s1.split_by_location(1), vec![s1]);
  }

  #[test]
  fn test_split_range_no_intersect() {
    let s1 = SRange::new(0, 1);
    let s2 = SRange::new(1, 1);
    assert_eq!(s1.split_by_range(&s2), vec![s1, s2]);
  }

  #[test]
  fn test_split_range_inside() {
    let s1 = SRange::new(0, 3);
    let s2 = SRange::new(1, 1);
    assert_eq!(
      s1.split_by_range(&s2),
      vec![SRange::new(0, 1), SRange::new(1, 1), SRange::new(2, 1)]
    );
  }

  #[test]
  fn test_split_range_intersect() {
    let s1 = SRange::new(0, 2);
    let s2 = SRange::new(1, 2);
    assert_eq!(
      s1.split_by_range(&s2),
      vec![SRange::new(0, 1), SRange::new(1, 1), SRange::new(2, 1)]
    );
  }

  #[test]
  fn test_split_range_filter_empty() {
    let s1 = SRange::new(0, 1);
    let s2 = SRange::new(0, 1);
    assert_eq!(s1.split_by_range(&s2), vec![s1]);
  }
}
