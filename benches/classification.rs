#[macro_use]
extern crate bencher;

use bencher::Bencher;
use bayesic::Bayesic;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

lazy_static! {
  static ref PARSED: HashMap<String, Vec<String>> = parsed_data_set();
  static ref WORDS: Regex = Regex::new(r"\b\w+\b").unwrap();
  static ref SMALL_TRAINED: Bayesic = small_trained();
  static ref LARGE_TRAINED: Bayesic = train(parsed_data_set());
}

fn path_to_words(path_str: &str) -> Vec<String> {
  let path = Path::new(path_str).to_path_buf();
  let bin = std::fs::read(path).ok().unwrap();
  let s = String::from_utf8(bin).ok().unwrap();
  let words: Vec<String> = WORDS.find_iter(&s).map(|m: regex::Match| String::from(m.as_str()) ).collect();
  return words;
}

fn train(data: HashMap<String, Vec<String>>) -> Bayesic {
  let mut bayesic = Bayesic::new();
  for (key, value) in data {
    bayesic.train(key, value)
  }
  bayesic.prune(0.1);
  return bayesic;
}

fn small_trained() -> Bayesic {
  let mut bayesic = Bayesic::new();
  bayesic.train("jojo".to_string(), path_to_words("priv/training/jojo_rabbit"));
  bayesic.train("jurassic_park".to_string(), path_to_words("priv/training/jurassic_park"));
  bayesic.train("jurassic_park_ii".to_string(), path_to_words("priv/training/jurassic_park_ii"));
  bayesic.train("jurassic_park_iii".to_string(), path_to_words("priv/training/jurassic_park_iii"));
  bayesic.train("kpax".to_string(), path_to_words("priv/training/kpax"));
  bayesic.prune(0.1);
  return bayesic;
}

fn parsed_data_set() -> HashMap<String, Vec<String>> {
  let path = Path::new("priv/training/imdb_titles.tsv").to_path_buf();
  let mut hash: HashMap<String, Vec<String>> = HashMap::new();
  let reader = BufReader::new(File::open(path).unwrap());
  let mut skip_header = true;
  for line in reader.lines() {
    if skip_header {
      skip_header = false;
    } else {
      let o = line.unwrap();
      let pieces: Vec<&str> = o.split("\t").collect();
      let tokens = WORDS.find_iter(&pieces[2]).map(|m: regex::Match| String::from(m.as_str())).collect();
      hash.insert(pieces[0].to_string(), tokens);
    }
  }
  return hash;
}

fn small_classify_one_word(bench: &mut Bencher) {
  bench.iter(|| {
    SMALL_TRAINED.classify(vec!("the".to_string()));
  })
}

fn small_classify_three_words(bench: &mut Bencher) {
  bench.iter(|| {
    SMALL_TRAINED.classify(vec!("the".to_string(), "a".to_string(), "with".to_string()));
  })
}

fn large_classify_one_word(bench: &mut Bencher) {
  bench.iter(|| {
    LARGE_TRAINED.classify(vec!("Silver".to_string()));
  })
}

fn large_classify_three_words(bench: &mut Bencher) {
  bench.iter(|| {
    // including The to simulate a token that has lots of potential matches
    LARGE_TRAINED.classify(vec!("The".to_string(), "Silver".to_string(), "Screen".to_string()));
  })
}
benchmark_group!(benches, small_classify_one_word, small_classify_three_words, large_classify_one_word, large_classify_three_words);
benchmark_main!(benches);