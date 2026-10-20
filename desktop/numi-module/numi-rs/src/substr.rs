use crate::*;

/// Helper type that stores substring reference and range
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Substr<'a> {
  str: &'a str,
  range: SRange,
}

impl<'a> Substr<'a> {
  // - init

  pub fn with_str(str: &'a str) -> Substr<'a> {
    Substr::new(str, SRange::new(0, str.len()))
  }

  pub fn with_empty_str(str: &str) -> Substr {
    Substr::new(str, SRange::default())
  }

  pub fn with_str_from(str: &str, start: usize) -> Substr {
    Substr::new(str, SRange::new(start, str.len() - start))
  }

  pub fn with_str_to(str: &str, len: usize) -> Substr {
    Substr::new(str, SRange::new(0, len))
  }

  pub fn new<T>(str: &'a str, range: T) -> Substr<'a>
  where
    T: Into<SRange> + std::fmt::Debug + std::clone::Clone,
  {
    debug_assert!(
      range.clone().into().end() <= str.len(),
      "Substr range '{:?}' out of bounds '{}'",
      range,
      str.len()
    );
    Substr {
      str,
      range: range.into(),
    }
  }

  // - merge

  /// Check if other substr points to the same substr
  pub fn can_merge(&self, other: &Substr) -> bool {
    if !self.is_same_str(other) {
      return false;
    }
    true
  }

  pub fn merge(&mut self, other: Substr) {
    self.assert_same_str(&other);
    let mut new_range = self.range;
    new_range.merge(other.range);
    self.set_range(new_range);
  }

  pub fn merge_serial(&mut self, other: Substr) {
    self.assert_same_str(&other);
    let mut new_range = self.range;
    new_range.merge_serial(other.range);
    self.set_range(new_range);
  }

  pub fn merge_disjoint(&mut self, other: Substr) {
    self.assert_same_str(&other);
    let mut new_range = self.range;
    new_range.merge_disjoint(other.range);
    self.set_range(new_range);
  }

  // - split

  pub fn split_by(&self, other: &'a Substr) -> Vec<Substr<'a>> {
    vec![self.clone(), other.clone()]
  }

  // - range

  pub fn set_range<T>(&mut self, range: T)
  where
    T: Into<SRange> + std::clone::Clone + std::fmt::Debug,
  {
    debug_assert!(
      range.clone().into().end() <= self.str.len(),
      "Range '{:?}' out of bounds '{}'",
      range,
      self.str.len()
    );
    self.range = range.into();
  }

  pub fn shift_range(&mut self, shift: isize) {
    let start = self.range.start as isize + shift;
    debug_assert!(
      start >= 0,
      "Shift range {:?} too much {:?}",
      self.range,
      shift
    );
    self.set_range(SRange::new(start as usize, self.range.len));
  }

  pub fn end(&self) -> usize {
    self.range.end()
  }

  pub fn is_empty(&self) -> bool {
    self.range.len == 0
  }

  pub fn range(&self) -> SRange {
    self.range
  }

  // - str

  pub fn str_in_range(&self) -> &'a str {
    &self.str[self.range.to_range()]
  }

  pub fn empty_substr(&self) -> Substr<'a> {
    Self::with_empty_str(self.str)
  }

  pub fn whole_str(&self) -> &'a str {
    self.str
  }

  fn assert_same_str(&self, other: &Substr) {
    debug_assert!(
      self.is_same_str(other),
      "append_substr should be applied to the same &str, got two different references"
    );
  }

  pub fn is_same_str(&self, other: &Self) -> bool {
    self.str == other.str
  }
}

impl<'a> From<&'a str> for Substr<'a> {
  fn from(s: &'a str) -> Substr<'a> {
    Substr::with_str(s)
  }
}

pub struct SubstrUtils {}

impl SubstrUtils {
  pub fn fill_spaces_in_range<'s, R>(substrs: &Vec<Substr<'s>>, irange: R) -> Vec<Substr<'s>>
  where
    R: Into<SRange>,
  {
    let mut res = vec![];
    let Some(first) = substrs.first() else {
      return res;
    };
    let str = first.whole_str();
    let range = irange.into();
    let mut i = range.start;

    for s in substrs {
      // push before the s range
      if s.range.start > i {
        res.push(Substr::new(str, i..s.range.start));
      }
      // push the s range
      res.push(s.clone());
      i = s.range.end();
    }
    // push the tail
    if range.end() > i {
      res.push(Substr::new(str, i..range.end()));
    }
    res
  }
}

pub trait SubstrOps {
  fn substr_join(&self, str: &str) -> String;
}

impl<'a> SubstrOps for [Substr<'a>] {
  fn substr_join(&self, str: &str) -> String {
    let res: Vec<&str> = self.iter().map(|s| s.str_in_range()).collect();
    res.join(str)
  }
}

#[cfg(test)]
mod susbstr_ops_test {
  use super::*;
  use crate::substr::Substr;

  #[test]
  fn test() {
    let text = "abc";
    let vec = vec![
      Substr::new(text, 0..1),
      Substr::new(text, 1..2),
      Substr::new(text, 2..3),
    ];
    assert_eq!(vec.substr_join(" "), "a b c");
    assert_eq!(vec[0..2].substr_join(" "), "a b");
  }

  #[test]
  fn test_fill_spaces_inside() {
    let text = "abc";
    let vec = vec![Substr::new(text, 1..2)];
    let expected = vec![
      Substr::new(text, 0..1),
      Substr::new(text, 1..2),
      Substr::new(text, 2..3),
    ];
    assert_eq!(SubstrUtils::fill_spaces_in_range(&vec, 0..3), expected);
  }

  #[test]
  fn test_fill_spaces_at_the_end() {
    let text = "abc";
    let vec = vec![Substr::new(text, 0..1)];
    let expected = vec![Substr::new(text, 0..1), Substr::new(text, 1..3)];
    assert_eq!(SubstrUtils::fill_spaces_in_range(&vec, 0..3), expected);
  }

  #[test]
  fn test_fill_spaces_at_the_beginning() {
    let text = "abc";
    let vec = vec![Substr::new(text, 2..3)];
    let expected = vec![Substr::new(text, 0..2), Substr::new(text, 2..3)];
    assert_eq!(SubstrUtils::fill_spaces_in_range(&vec, 0..3), expected);
  }

  #[test]
  fn test_fill_no_empty_spaces() {
    let text = "abc";
    let vec = vec![Substr::new(text, 0..1), Substr::new(text, 1..2)];
    let expected = vec![
      Substr::new(text, 0..1),
      Substr::new(text, 1..2),
      Substr::new(text, 2..3),
    ];
    assert_eq!(SubstrUtils::fill_spaces_in_range(&vec, 0..3), expected);
  }

  #[test]
  fn test_fill_multiple_substrs() {
    let text = "abcde";
    let vec = vec![Substr::new(text, 1..2), Substr::new(text, 3..4)];
    let expected = vec![
      Substr::new(text, 0..1),
      Substr::new(text, 1..2),
      Substr::new(text, 2..3),
      Substr::new(text, 3..4),
      Substr::new(text, 4..5),
    ];
    assert_eq!(SubstrUtils::fill_spaces_in_range(&vec, 0..5), expected);
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  // - init

  #[test]
  fn test_with_str() {
    let substr = Substr::with_str("ab");
    assert_eq!(substr, Substr::new("ab", 0..2));
  }

  #[test]
  fn test_str_from() {
    let substr = Substr::with_str_from("abc", 2);
    assert_eq!(substr, Substr::new("abc", 2..3));
  }

  #[test]
  fn test_str_to() {
    let substr = Substr::with_str_to("abc", 2);
    assert_eq!(substr, Substr::new("abc", 0..2));
  }

  #[test]
  #[should_panic]
  fn test_should_panic_on_oob() {
    Substr::new("a", 0..2);
  }

  // - merge

  #[test]
  fn test_can_merge() {
    let substr1 = Substr::new("abc", 0..1);
    let substr2 = Substr::new("abc", 1..2);
    let substr3 = Substr::new("cde", 1..2);
    assert!(substr1.can_merge(&substr2));
    assert!(!substr1.can_merge(&substr3));
  }

  #[test]
  fn test_merge() {
    let str = "abc";
    let mut s1 = Substr::new(str, 0..1);
    s1.merge(Substr::new(str, 1..2));
    assert_eq!(s1.str_in_range(), "ab");
  }

  #[test]
  #[should_panic]
  fn test_merge_should_panic_on_different_strs() {
    Substr::with_str("a").merge(Substr::with_str("b"))
  }

  // - str

  #[test]
  fn test_str_in_range() {
    let substr = Substr::new("str", 1..2);
    assert_eq!(substr.str_in_range(), "t");
  }

  #[test]
  fn test_empty_substr() {
    let substr = Substr::new("str", 1..2).empty_substr();
    assert_eq!(substr.str, "str");
    assert_eq!(substr.range, SRange::from(0..0));
  }

  // - range

  #[test]
  fn test_set_range() {
    let mut substr = Substr::new("abc", 0..1);
    substr.set_range(1..2);
    assert_eq!(substr.range, SRange::new(1, 1));
  }

  #[test]
  #[should_panic]
  fn test_set_range_should_panic_on_oob() {
    let mut substr = Substr::new("abc", 0..1);
    substr.set_range(3..4);
  }

  #[test]
  fn test_split_for_no_intersect() {
    let str = "abc";
    let s1 = Substr::new(str, 0..1);
    let s2 = Substr::new(str, 2..3);
    let result = s1.split_by(&s2);
    assert_eq!(result, vec![s1.clone(), s2.clone()]);
  }
}
