use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub struct StringsParser {
  pub dict: HashMap<String, String>,
  regex: Regex,
}

impl Default for StringsParser {
  fn default() -> Self {
    Self {
      dict: HashMap::new(),
      regex: Regex::new(r#""(.*)"\s*=\s*"(.*)""#).unwrap(),
    }
  }
}

impl StringsParser {
  pub fn has_some(&self) -> bool {
    !self.dict.is_empty()
  }

  pub fn parse_file(&mut self, file_name: &str) {
    let Ok(mut file) = File::open(file_name) else {
      return;
    };
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    self.parse_text(&content);
  }

  pub fn parse_text(&mut self, str: &str) {
    for line in str.lines() {
      if let Some((k, v)) = self.parse_line(line) {
        self.dict.insert(k.to_string(), v.to_string());
      }
    }
  }

  pub fn get_strings(&self, str: &str) -> Vec<&str> {
    if let Some(v) = self.get_string(str) {
      return v.split(',').collect();
    }
    Vec::new()
  }

  pub fn get_strings_lo(&self, str: &str) -> Vec<String> {
    if let Some(v) = self.get_string_lo(str) {
      return v.split(',').map(|s| s.to_string()).collect();
    }
    Vec::new()
  }

  pub fn get_string(&self, str: &str) -> Option<&str> {
    if let Some(v) = self.dict.get(str) {
      return Some(v.as_str());
    }
    None
  }

  pub fn get_string_lo(&self, str: &str) -> Option<String> {
    if let Some(v) = self.dict.get(str) {
      return Some(v.to_lowercase());
    }
    None
  }

  pub fn parse_line<'a>(&self, str: &'a str) -> Option<(&'a str, &'a str)> {
    let captures = self.regex.captures(str)?;
    let m1 = captures.get(1)?;
    let m2 = captures.get(2)?;
    Some((m1.as_str(), m2.as_str()))
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test_parse_text() {
    let str = r#"
"a" = "b";

/* comment */
"cc" = "dd";
			"#;
    let mut p = StringsParser::default();
    p.parse_text(str);
    assert_eq!(p.get_string("a"), Some("b"));
    assert_eq!(p.get_string("cc"), Some("dd"));
    assert_eq!(p.dict.len(), 2);
  }

  #[test]
  fn test_get_strings() {
    let mut p = StringsParser::default();
    p.parse_text(r#""a"="b,c""#);
    assert_eq!(p.get_strings("a"), vec!["b", "c"]);
  }

  #[test]
  fn test_parse_line() {
    let sp = StringsParser::default();
    assert_eq!(sp.parse_line(r#""a" = "b";"#).unwrap(), ("a", "b"));
    assert_eq!(sp.parse_line(r#" "a"= "b";"#).unwrap(), ("a", "b"));
    assert_eq!(sp.parse_line(r#"a=b;"#), None);
  }
}
