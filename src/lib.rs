use std::collections::{HashMap,HashSet};
//use std::collections::hash_map::Keys;

struct Trainer {
  by_token: HashMap<String, HashSet<String>>,
  by_class: HashMap<String, HashSet<String>>,
}

#[cfg(test)]
mod tests {
  extern crate pretty_assertions;

  use std::path::{Path,PathBuf};
  use lazy_static::lazy_static;
  use regex::Regex;

  lazy_static! {
    static ref WORDS: Regex = Regex::new(r"\b\w+\b").unwrap();
  }

  fn path_to_words(path: &PathBuf) -> Vec<String> {
    let bin = std::fs::read(path).ok().unwrap();
    let s = String::from_utf8(bin).ok().unwrap();
    let words: Vec<String> = WORDS.find_iter(&s).map(|m: regex::Match| String::from(m.as_str()) ).collect();
    return words;
  }

  #[test]
  fn parsing_file_words() {
    let strs = vec!("A", "young", "boy", "in", "Hitler", "s", "army", "finds", "out", "his", "mother", "is", "hiding", "a", "Jewish", "girl", "in", "their", "home");
    let expected: Vec<String> = strs.iter().map(|s| s.to_string()).collect();
    assert_eq!(path_to_words(&Path::new("priv/training/jojo_rabbit").to_path_buf()), expected)
  }
}
