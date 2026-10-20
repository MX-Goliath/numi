use super::*;
use std::fmt::{self, Debug};

#[derive(PartialEq, Eq, Clone)]
pub struct AttrText<'a> {
  pub substr: Substr<'a>,
  // Markup is absolute within substr
  pub markup: Vec<AttrEl>,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct AttrEl {
  // The range is absolute within Substr whole str, as there's no need
  // to perform any kind of computations/index shift when combining strings
  pub range: SRange,
  pub kind: AttrKind,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum AttrKind {
  Number,
  Op,
  Unit,
  Func,
  Var,
}

impl fmt::Display for AttrKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      AttrKind::Number => "Number",
      AttrKind::Op => "Op",
      AttrKind::Unit => "Unit",
      AttrKind::Func => "Func",
      AttrKind::Var => "Var",
    };
    write!(f, "{}", s)
  }
}

impl AttrEl {
  pub fn new(range: SRange, kind: AttrKind) -> Self {
    Self { range, kind }
  }
}

impl<'a> AttrText<'a> {
  // - init

  pub fn with_empty_substr<T>(in_substr: T) -> Self
  where
    T: Into<Substr<'a>>,
  {
    Self::with_substr(in_substr.into().empty_substr())
  }

  pub fn with_substr<T>(isubstr: T) -> Self
  where
    T: Into<Substr<'a>>,
  {
    Self::new(isubstr.into(), Vec::new())
  }

  pub fn with_substr_of_kind<T>(isubstr: T, kind: AttrKind) -> Self
  where
    T: Into<Substr<'a>>,
  {
    let substr = isubstr.into();
    Self::with_substr_of_kind_in_range(substr.clone(), kind, substr.range())
  }

  pub fn of_kind(&self, kind: AttrKind) -> Self {
    Self::with_substr_of_kind(self.substr.clone(), kind)
  }

  pub fn with_substr_of_kind_in_range<T, R>(isubstr: T, kind: AttrKind, in_range: R) -> Self
  where
    T: Into<Substr<'a>>,
    R: Into<SRange>,
  {
    let substr = isubstr.into();
    let range = in_range.into();
    let attrs = vec![AttrEl::new(range, kind)];
    Self::new(substr, attrs)
  }

  pub fn new(substr: Substr<'a>, markup: Vec<AttrEl>) -> AttrText {
    let mut attr_text = AttrText {
      substr,
      markup: vec![],
    };
    // Use API so invalid markup can be asserted in debug
    attr_text.append_markup(markup);
    attr_text
  }

  // - merge

  pub fn can_merge(&self, other: &AttrText<'a>) -> bool {
    if !self.substr.can_merge(&other.substr) {
      return false;
    }
    let mut substr = self.substr.clone();
    substr.merge(other.substr.clone());
    let text = AttrText {
      substr,
      markup: self.markup.clone(),
    };
    if !text.can_append_markup(&other.markup) {
      return false;
    }
    true
  }

  // Technically all merge methods are the same,
  // the only difference is the checks that are performed
  // and when those trigger assertions (debug)

  pub fn merge(&mut self, other: AttrText<'a>) {
    self.substr.merge(other.substr);
    self.append_markup(other.markup);
  }

  pub fn merge_serial(&mut self, other: AttrText<'a>) {
    self.substr.merge_serial(other.substr);
    self.append_markup(other.markup);
  }

  pub fn merge_disjoint(&mut self, other: AttrText<'a>) {
    self.substr.merge_disjoint(other.substr);
    self.append_markup(other.markup);
  }

  // - append

  pub fn append_markup(&mut self, els: Vec<AttrEl>) {
    for el in els {
      self.append_markup_element(el);
    }
  }

  pub fn append_markup_element(&mut self, el: AttrEl) {
    debug_assert!(
      self.can_append_markup_element(&el),
      "Can't add markup element '{el:?}' to self '{self:?}'"
    );
    self.markup.push(el);
  }

  pub fn can_append_markup(&self, els: &[AttrEl]) -> bool {
    for el in els {
      if !self.can_append_markup_element(el) {
        return false;
      }
    }
    true
  }

  pub fn can_append_markup_element(&self, el: &AttrEl) -> bool {
    if !el.range.is_in(self.substr.range()) {
      return false;
    }
    if el.range.is_empty() {
      return false;
    }
    for e in &self.markup {
      if e.range.is_intersect(&el.range) {
        return false;
      }
    }
    true
  }

  // - substr

  pub fn str_in_range(&self) -> &'a str {
    self.substr.str_in_range()
  }

  pub fn whole_str(&self) -> &'a str {
    self.substr.whole_str()
  }

  pub fn is_empty(&self) -> bool {
    self.substr.is_empty()
  }

  pub fn range(&self) -> SRange {
    self.substr.range()
  }
}

impl<'a> Debug for AttrText<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut i_start = self.substr.range().start;
    let mut strs = vec![];
    for el in &self.markup {
      // Add previous string part without any attributes
      if el.range.start > i_start {
        let r = i_start..el.range.start;
        let s = Substr::new(self.substr.whole_str(), r);
        strs.push(format!(r#""{}""#, s.str_in_range()));
      }
      let s = Substr::new(self.substr.whole_str(), el.range);
      let el_str = format!(r#""{}"[{:?}]"#, s.str_in_range(), el.kind);
      strs.push(el_str);
      i_start = el.range.end();
    }
    if i_start < self.substr.range().end() {
      let r = i_start..self.substr.range().end();
      let s = Substr::new(self.substr.whole_str(), r);
      strs.push(format!(r#""{}""#, s.str_in_range()));
    }
    let format_str = strs.join("; ");
    f.write_fmt(format_args!("AttrText {format_str}"))?;
    Result::Ok(())
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  // - init

  #[test]
  fn test_with_empty_substr() {
    let text = AttrText::with_empty_substr("abc");
    assert!(text.substr.is_empty());
    assert!(text.markup.is_empty());
  }

  #[test]
  fn test_with_substr() {
    let text = AttrText::with_substr("abc");
    assert_eq!(text.substr, Substr::with_str("abc"));
    assert!(text.markup.is_empty());
  }

  #[test]
  fn test_with_substr_of_kind() {
    let text = AttrText::with_substr_of_kind("abc", AttrKind::Number);
    assert_eq!(text.substr, Substr::with_str("abc"));
    assert_eq!(
      text.markup,
      vec![AttrEl::new(SRange::new(0, 3), AttrKind::Number)]
    );
  }

  #[test]
  fn test_with_substr_of_kind_in_range() {
    let text = AttrText::with_substr_of_kind_in_range("abc", AttrKind::Number, 1..2);
    assert_eq!(text.substr, Substr::with_str("abc"));
    assert_eq!(
      text.markup,
      vec![AttrEl::new(SRange::new(1, 1), AttrKind::Number)]
    );
  }

  #[test]
  #[should_panic]
  fn test_panic_on_invalid_el_range() {
    AttrText::new(
      Substr::with_str("a"),
      vec![AttrEl::new(SRange::new(1, 1), AttrKind::Number)],
    );
  }

  #[test]
  #[should_panic]
  fn test_panic_on_invalid_intersect() {
    AttrText::new(
      Substr::with_str("a"),
      vec![
        AttrEl::new(SRange::new(0, 1), AttrKind::Number),
        AttrEl::new(SRange::new(0, 1), AttrKind::Op),
      ],
    );
  }

  // - merge

  #[test]
  fn test_merge() {
    let t1 = AttrText::with_substr_of_kind_in_range("abc", AttrKind::Number, 1..2);
    let t2 = AttrText::with_substr_of_kind_in_range("abc", AttrKind::Op, 2..3);
    assert!(t1.can_merge(&t2));
  }

  // - append

  #[test]
  fn test_append() {
    let str = "ab";
    let mut text1 = AttrText::with_substr_of_kind(Substr::new(str, 0..1), AttrKind::Number);
    let text2 = AttrText::with_substr_of_kind(Substr::new(str, 1..2), AttrKind::Op);
    text1.merge(text2);
    let expected = AttrText::new(
      Substr::with_str(str),
      vec![
        AttrEl::new(SRange::from_range(0..1), AttrKind::Number),
        AttrEl::new(SRange::from_range(1..2), AttrKind::Op),
      ],
    );
    assert_eq!(text1, expected)
  }

  #[test]
  fn test_can_append_element() {
    let text = AttrText::with_substr_of_kind_in_range("1+2", AttrKind::Op, SRange::new(1, 1));
    assert!(text.can_append_markup_element(&AttrEl::new(SRange::new(0, 1), AttrKind::Number)));
    assert!(text.can_append_markup_element(&AttrEl::new(SRange::new(2, 1), AttrKind::Number)));
    assert!(!text.can_append_markup_element(&AttrEl::new(SRange::new(1, 1), AttrKind::Number)));
    assert!(!text.can_append_markup_element(&AttrEl::new(SRange::new(0, 2), AttrKind::Number)));
    assert!(!text.can_append_markup_element(&AttrEl::new(SRange::new(3, 1), AttrKind::Number)));
    assert!(!text.can_append_markup_element(&AttrEl::new(SRange::new(2, 2), AttrKind::Number)));
  }

  #[test]
  fn test_debug() {
    let text = AttrText::new(
      Substr::with_str("abcde"),
      vec![
        AttrEl::new(SRange::new(1, 1), AttrKind::Number),
        AttrEl::new(SRange::new(3, 1), AttrKind::Unit),
      ],
    );
    assert_eq!(
      format!("{:?}", &text),
      r#"AttrText "a"; "b"[Number]; "c"; "d"[Unit]; "e""#
    );
  }
}
