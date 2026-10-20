pub trait StrExt {
  fn index_in(&self, other: &str) -> isize;
  fn string_without_whitespaces(&self) -> String;
}

impl StrExt for str {
  fn index_in(&self, other: &str) -> isize {
    let my_ptr = self.as_ptr();
    let other_ptr = other.as_ptr();
    unsafe { my_ptr.offset_from(other_ptr) }
  }

  fn string_without_whitespaces(&self) -> String {
    let no_whitespace = self.chars().filter(|&c| !c.is_whitespace()).collect();
    no_whitespace
  }
}

#[cfg(test)]
mod test_str_ext {

  use super::*;

  #[test]
  fn test_index_in() {
    let str = "abc";
    let substr = &str[1..2];
    assert_eq!(substr.index_in(str), 1);
  }

  #[test]
  fn test_no_whitespaces() {
    assert_eq!("a b c".string_without_whitespaces(), "abc");
  }
}
