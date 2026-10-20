use crate::strings_parser::StringsParser;
use log::{debug, error};
use std::{fs::File, io::Read};

#[derive(Default)]
pub struct Localizer {
  en_parser: StringsParser,
  loc_parser: StringsParser,
}

impl Localizer {
  pub fn new() -> Self {
    Self {
      en_parser: StringsParser::default(),
      loc_parser: StringsParser::default(),
    }
  }

  pub fn load_en_strings(&mut self) {
    let strings = include_str!("../assets/en.strings");
    self.en_parser.parse_text(strings);
  }

  pub fn add_loc_string(&mut self, key: &str, string: &str) {
    self
      .loc_parser
      .dict
      .insert(key.to_string(), string.to_string());
  }

  pub fn add_en_string(&mut self, key: &str, string: &str) {
    self
      .en_parser
      .dict
      .insert(key.to_string(), string.to_string());
  }

  pub fn parse_loc_file(&mut self, filepath: &str) {
    if let Ok(mut f) = File::open(filepath) {
      let mut s = String::new();
      if f.read_to_string(&mut s).is_ok() {
        self.parse_loc_content(&s);
        debug!("Parsed loc file: {:}", filepath);
      } else {
        error!("Failed to read localization file {:}", filepath);
      }
    } else {
      error!("Failed to open localization file {:}", filepath);
    }
  }

  pub fn parse_loc_content(&mut self, content: &str) {
    self.loc_parser.parse_text(content);
  }

  pub fn has_loc_content(&self) -> bool {
    self.loc_parser.has_some()
  }

  pub fn get_strings_lo(&self, str: &str) -> Vec<String> {
    let res = self
      .get_strings(str)
      .iter()
      .map(|s| s.to_lowercase())
      .collect();
    res
  }

  pub fn get_loc_strings(&self, str: &str) -> Vec<&str> {
    self.loc_parser.get_strings(str)
  }

  pub fn get_en_strings(&self, str: &str) -> Vec<&str> {
    self.en_parser.get_strings(str)
  }

  pub fn get_strings(&self, str: &str) -> Vec<&str> {
    let mut res = self.loc_parser.get_strings(str);
    res.append(&mut self.en_parser.get_strings(str));
    res
  }

  pub fn get_loc_string(&self, str: &str) -> Option<&str> {
    self.loc_parser.get_string(str)
  }

  pub fn get_en_string(&self, str: &str) -> Option<&str> {
    self.en_parser.get_string(str)
  }

  pub fn get_string(&self, str: &str) -> Option<&str> {
    if self.loc_parser.get_string(str).is_some() {
      return self.loc_parser.get_string(str);
    }
    self.en_parser.get_string(str)
  }
}
