use std::collections::HashMap;

use crate::{
  srange::SRange,
  str_ranger::StrRanger,
  substr::{Substr, SubstrOps},
};

#[derive(Clone)]
pub struct PhraseDetector {
  str_ranger: StrRanger,
  phrases: HashMap<String, String>,
  lowercase_search: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PhraseMatch<'a, 'b> {
  pub id: &'a str,
  pub substr: Substr<'b>,
}

impl<'a, 'b> PhraseMatch<'a, 'b> {
  fn new(id: &'a str, substr: Substr<'b>) -> Self {
    Self { id, substr }
  }
}

impl Default for PhraseDetector {
  fn default() -> Self {
    Self {
      str_ranger: StrRanger::new(),
      phrases: HashMap::new(),
      lowercase_search: true,
    }
  }
}

impl PhraseDetector {
  pub fn new(str_ranger: StrRanger, lowercase_search: bool) -> Self {
    Self {
      str_ranger,
      phrases: HashMap::new(),
      lowercase_search,
    }
  }

  pub fn register_phrases(&mut self, phrases: &Vec<&str>) {
    for phrase in phrases {
      self.add_phrase_for(phrase, phrase);
    }
  }

  pub fn add_normalized_phrases_for(&mut self, id: &str, phrases: &Vec<&str>) {
    for phrase in phrases {
      self.add_normalized_phrase_for(id, phrase);
    }
  }

  pub fn add_phrases_for(&mut self, id: &str, phrases: &Vec<&str>) {
    for phrase in phrases {
      self.add_phrase_for(id, phrase);
    }
  }

  pub fn add_lo_phrases_for(&mut self, id: &str, phrases: &Vec<&str>) {
    for phrase in phrases {
      self.add_phrase_for(id, &phrase.to_lowercase());
    }
  }

  pub fn add_phrase_for(&mut self, id: &str, phrase: &str) {
    self.add_normalized_phrase_for(id, &self.phrase_normalized(phrase));
  }

  pub fn add_normalized_phrase_for(&mut self, id: &str, phrase: &str) {
    if !self.phrases.contains_key(phrase) {
      self.phrases.insert(phrase.to_owned(), id.to_owned());
    } else {
      // println!("Ignoring duplicate phrase {} for {}", phrase, id);
    }
  }

  pub fn id_for_phrase<'s, S>(&self, phrase: S) -> Option<&str>
  where
    S: Into<&'s str>,
  {
    if let Some(id) = self.phrases.get(&self.phrase_normalized(phrase.into())) {
      return Some(id);
    }
    None
  }

  pub fn id_for_normalized_phrase<'s, S>(&self, phrase: S) -> Option<&str>
  where
    S: Into<&'s str>,
  {
    if let Some(id) = self.phrases.get(phrase.into()) {
      return Some(id);
    }
    None
  }

  pub fn phrase_normalized(&self, phrase: &str) -> String {
    self.str_ranger.words(phrase).join(" ")
  }

  pub fn matches<'a, 's, S>(&'a self, into_substr: S) -> Vec<PhraseMatch<'a, 's>>
  where
    S: Into<Substr<'s>>,
  {
    let mut res = vec![];
    let substr = into_substr.into();
    let s_words = self.str_ranger.words_substr(&substr);
    // Scan from left to right and select the longest phrase at each position.
    // This keeps earlier units (for example `Hz`) when a later multi-word unit
    // (for example `angular frequency`) is present in the same expression.
    let mut i = 0;
    while i < s_words.len() {
      let mut matched = None;
      for len in (1..=s_words.len() - i).rev() {
        let phrase = s_words[i..i + len].substr_join(" ");
        matched = self
          .id_for_normalized_phrase(phrase.as_str())
          .or_else(|| {
            if self.lowercase_search {
              self.id_for_normalized_phrase(phrase.to_lowercase().as_str())
            } else {
              None
            }
          })
          .map(|id| (id, len));
        if matched.is_some() {
          break;
        }
      }
      if let Some((id, len)) = matched {
        let to_word = i + len;
        let range =
          SRange::from_range(s_words[i].range().start..s_words[to_word - 1].range().end());
        res.push(PhraseMatch {
          id,
          substr: Substr::new(substr.whole_str(), range),
        });
        i = to_word;
      } else {
        i += 1;
      }
    }
    res
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test_id_for_single_word_phrase() {
    let mut pd = PhraseDetector::new(StrRanger::new(), true);
    pd.add_phrase_for("id", "phrase");
    assert_eq!(pd.id_for_phrase("phrase").unwrap(), "id");
  }

  #[test]
  fn test_id_for_multi_word_phrase() {
    let mut pd = PhraseDetector::new(StrRanger::new(), true);
    pd.add_phrase_for("id", "a   b");
    assert_eq!(pd.id_for_phrase("a  b").unwrap(), "id");
  }

  #[test]
  fn test_no_matches() {
    let mut pd = PhraseDetector::new(StrRanger::new(), true);
    pd.add_phrase_for("id", "a");
    assert_eq!(pd.matches("b"), vec![]);
  }

  #[test]
  fn test_single_match_single_word() {
    let mut pd = PhraseDetector::new(StrRanger::new(), true);
    pd.add_phrase_for("id", "a");
    assert_eq!(
      pd.matches("a"),
      vec![PhraseMatch::new("id", Substr::new("a", 0..1))]
    );
  }

  #[test]
  fn test_single_match_mutli_word() {
    let mut pd = PhraseDetector::new(StrRanger::new(), true);
    pd.add_phrase_for("id", "a b");
    assert_eq!(
      pd.matches("a  b"),
      vec![PhraseMatch::new("id", Substr::new("a  b", 0..4))]
    );
  }

  #[test]
  fn test_single_match_mutli_word_at_some_pos() {
    let mut pd = PhraseDetector::new(StrRanger::new(), true);
    pd.add_phrase_for("id", "b c");
    assert_eq!(
      pd.matches("a b c"),
      vec![PhraseMatch::new("id", Substr::new("a b c", 2..5))]
    );
  }

  #[test]
  fn test_multi_matches_single_word() {
    let mut pd = PhraseDetector::new(StrRanger::new(), true);
    pd.add_phrase_for("id1", "a");
    pd.add_phrase_for("id2", "b");
    let text = "a b";
    assert_eq!(
      pd.matches(text),
      vec![
        PhraseMatch::new("id1", Substr::new(text, 0..1)),
        PhraseMatch::new("id2", Substr::new(text, 2..3))
      ]
    );
  }

  #[test]
  fn test_longest_match_win() {
    let mut pd = PhraseDetector::new(StrRanger::new(), true);
    pd.add_phrase_for("id1", "b c");
    pd.add_phrase_for("id2", "b");
    assert_eq!(
      pd.matches("a b c"),
      vec![PhraseMatch::new("id1", Substr::new("a b c", 2..5))]
    );
  }

  #[test]
  fn test_case_insensitive() {
    let mut pd = PhraseDetector::new(StrRanger::new(), true);
    pd.add_phrase_for("id", "a");
    assert_eq!(
      pd.matches("A"),
      vec![PhraseMatch::new("id", Substr::new("A", 0..1))]
    );
  }
}
