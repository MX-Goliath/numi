use crate::{substr::SubstrUtils, *};
use regex::Regex;

#[derive(Clone)]
pub struct StrRanger {
  space_regex: Regex,
  symbols_regex: Regex,
  digits_regex: Regex,
}

impl Default for StrRanger {
  fn default() -> StrRanger {
    StrRanger {
      space_regex: Regex::new(r"\S+").unwrap(),
      symbols_regex: Regex::new(r"[\$\+\-\*/!\^%():]").unwrap(),
      digits_regex: Regex::new(r"[\d]+").unwrap(),
    }
  }
}

/// Find substrings
impl StrRanger {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn strs_for_regex<'a>(&self, str: &'a str, regex: &Regex) -> Vec<&'a str> {
    self
      .substrs_for_regex(str, regex)
      .iter()
      .map(|s| s.str_in_range())
      .collect()
  }

  pub fn substrs_for_regex<'a>(&self, str: &'a str, regex: &Regex) -> Vec<Substr<'a>> {
    self.substrs_for_regex_in_substr(&Substr::with_str(str), regex)
  }

  pub fn words<'a>(&self, str: &'a str) -> Vec<&'a str> {
    self
      .words_substr(&Substr::with_str(str))
      .iter()
      .map(|s| s.str_in_range())
      .collect()
  }

  pub fn words_substr<'a>(&self, substr: &Substr<'a>) -> Vec<Substr<'a>> {
    let mut res = vec![substr.clone()];
    res = self.substrs_for_substrs(res, |s| {
      self.substrs_for_regex_in_substr(s, &self.space_regex)
    });
    res = self.substrs_for_substrs(res, |s| self.regex_split(s, &self.symbols_regex));
    res = self.substrs_for_substrs(res, |s| self.regex_split(s, &self.digits_regex));
    res
  }

  pub fn regex_split<'a>(&self, substr: &Substr<'a>, regex: &Regex) -> Vec<Substr<'a>> {
    let regex_substrs: Vec<Substr> = self.substrs_for_regex_in_substr(substr, regex);
    SubstrUtils::fill_spaces_in_range(&regex_substrs, substr.range())
  }

  pub fn substrs_for_regex_in_substr<'a>(
    &self,
    substr: &Substr<'a>,
    regex: &Regex,
  ) -> Vec<Substr<'a>> {
    let mut result: Vec<Substr> = Vec::new();
    let whole_str = substr.whole_str();
    for m in regex.find_iter(substr.str_in_range()) {
      let mut range = SRange::from_range(m.range());
      range.start += substr.range().start;
      result.push(Substr::new(whole_str, range));
    }
    result
  }

  pub fn substrs_for_substrs<'a, F>(&self, substrs: Vec<Substr<'a>>, f: F) -> Vec<Substr<'a>>
  where
    F: Fn(&Substr<'a>) -> Vec<Substr<'a>>,
  {
    let mut res = vec![];
    for s in substrs {
      let mut fn_substrs = f(&s);
      if !fn_substrs.is_empty() {
        res.append(&mut fn_substrs);
      } else {
        res.push(s);
      }
    }
    res
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_regex_in_substr() {
    let text = "x1d";
    let detector = StrRanger::new();
    let expected = vec![Substr::new(text, 1..2)];
    assert_eq!(
      detector.substrs_for_regex_in_substr(&Substr::new(text, 1..3), &Regex::new("\\d+").unwrap()),
      expected
    );
  }

  #[test]
  fn test_regex_in_str_empty() {
    let detector = StrRanger::new();
    let expected: Vec<Substr> = Vec::new();
    assert_eq!(
      detector.substrs_for_regex("", &Regex::new("d").unwrap()),
      expected
    );
  }

  #[test]
  fn test_regex_in_str() {
    let text = "x1d";
    let detector = StrRanger::new();
    let expected = vec![Substr::new(text, 1..2)];
    assert_eq!(
      detector.substrs_for_regex(text, &Regex::new("\\d+").unwrap()),
      expected
    );
  }

  #[test]
  fn test_strs_for_regex() {
    let text = "x1d";
    let detector = StrRanger::new();
    let expected = vec!["1"];
    assert_eq!(
      detector.strs_for_regex(text, &Regex::new("\\d+").unwrap()),
      expected
    );
  }

  #[test]
  fn test_sing_word_substr() {
    let text = "ab";
    let strranger = StrRanger::new();
    let expect = vec![Substr::new(text, 0..2)];
    assert_eq!(strranger.words_substr(&Substr::with_str(text)), expect);
  }

  #[test]
  fn test_words_substr() {
    let text = "a b";
    let strranger = StrRanger::new();
    let expect = vec![Substr::new(text, 0..1), Substr::new(text, 2..3)];
    assert_eq!(strranger.words_substr(&Substr::with_str(text)), expect);
  }

  #[test]
  fn test_words_between_spaces() {
    let sr = StrRanger::new();
    assert_eq!(sr.words("a b"), vec!["a", "b"]);
  }

  #[test]
  fn test_words_and_symbols() {
    let sr = StrRanger::new();
    assert_eq!(sr.words("a$"), vec!["a", "$"]);
  }

  #[test]
  fn test_more_symbols() {
    let sr = StrRanger::new();
    assert_eq!(
      sr.words("+*-/\\()"),
      vec!["+", "*", "-", "/", "\\", "(", ")"]
    );
  }

  #[test]
  fn test_words_complex() {
    assert_eq!(
      StrRanger::new().words("20$in!eur:times25"),
      vec!["20", "$", "in", "!", "eur", ":", "times", "25"]
    );
  }

  #[test]
  fn test_regex_split() {
    let text = "x1d";
    let detector = StrRanger::new();
    let expected = vec![
      Substr::new(text, 0..1),
      Substr::new(text, 1..2),
      Substr::new(text, 2..3),
    ];
    assert_eq!(
      detector.regex_split(&Substr::new(text, 0..3), &Regex::new("\\d+").unwrap()),
      expected
    );
  }

  #[test]
  fn test_regex_split_2() {
    let regex = Regex::new(r"[\$]").unwrap();
    let text = "a$b";
    let strranger = StrRanger::new();
    assert_eq!(
      strranger.substrs_for_regex(text, &regex),
      vec![Substr::new(text, 1..2)]
    );

    let expect_split = vec![
      Substr::new(text, 0..1),
      Substr::new(text, 1..2),
      Substr::new(text, 2..3),
    ];
    let res = strranger.regex_split(&Substr::with_str(text), &regex);
    assert_eq!(res, expect_split);
  }
}

/// Should be used to mix/merge/inverse substr ranges
/// Higher level intent to be used in word/phrase detector
pub struct StrMixer {}

impl StrMixer {
  pub fn inverse_substr_vec<'a>(&self, substrs: Vec<Substr<'a>>) -> Vec<Substr<'a>> {
    let mut i_rng = SRange::new(0, 0);
    let mut result = vec![];
    let mut whole_str: Option<&str> = None;

    // Iterates over all substr, store substr before if has length
    for substr in substrs {
      whole_str = Option::Some(substr.whole_str());
      i_rng.len = substr.range().start - i_rng.start;
      if i_rng.len > 0 {
        let substr = Substr::new(whole_str.unwrap(), i_rng);
        result.push(substr);
      }
      i_rng = SRange::new(substr.range().end(), 0);
    }

    // Store rightmost substr if needed
    if let Some(whole_str) = whole_str {
      if i_rng.start < whole_str.len() {
        i_rng.len = whole_str.len() - i_rng.start;
        let substr = Substr::new(whole_str, i_rng);
        result.push(substr);
      }
    }

    result
  }
}

#[cfg(test)]
mod str_mixer_test {
  use crate::{str_ranger::StrMixer, substr::Substr};

  #[test]
  fn test_inverse() {
    let str = "abc";
    let s1 = Substr::new(str, 0..1);
    let s2 = Substr::new(str, 1..2);
    let s3 = Substr::new(str, 2..3);
    let result = StrMixer {}.inverse_substr_vec(vec![s2]);
    assert_eq!(result, vec![s1, s3]);
  }

  #[test]
  fn test_inverse_start() {
    let str = "ab";
    let s1 = Substr::new(str, 0..1);
    let s2 = Substr::new(str, 1..2);
    let result = StrMixer {}.inverse_substr_vec(vec![s1]);
    assert_eq!(result, vec![s2]);
  }

  #[test]
  fn test_inverse_end() {
    let str = "ab";
    let s1 = Substr::new(str, 0..1);
    let s2 = Substr::new(str, 1..2);
    let result = StrMixer {}.inverse_substr_vec(vec![s2]);
    assert_eq!(result, vec![s1]);
  }

  #[test]
  fn test_inverse_for_empty() {
    let result = StrMixer {}.inverse_substr_vec(vec![]);
    assert_eq!(result, vec![]);
  }

  #[test]
  fn test_inverse_whole() {
    let str = "a";
    let s1 = Substr::new(str, 0..1);
    let result = StrMixer {}.inverse_substr_vec(vec![s1]);
    assert_eq!(result, vec![]);
  }
}
