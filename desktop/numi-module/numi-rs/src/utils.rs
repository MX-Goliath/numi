pub trait VecStrToString {
  fn to_strings(&self) -> Vec<String>;
}

impl VecStrToString for Vec<&str> {
  fn to_strings(&self) -> Vec<String> {
    self.iter().map(|m| m.to_string()).collect()
  }
}

pub trait VecStringToStr {
  fn to_strs(&self) -> Vec<&str>;
}

impl VecStringToStr for Vec<String> {
  fn to_strs(&self) -> Vec<&str> {
    self.iter().map(|m| m.as_str()).collect()
  }
}
