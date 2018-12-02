use itertools::iproduct;
use std::collections::HashMap;

fn main() {
  let input = std::fs::read_to_string("input1.txt").unwrap();
  println!("{}", calculate_cheksum(&input));
  println!("{}", find_common_letters(&input));
}

fn calculate_cheksum(input: &str) -> u32 {
  let mut doubles = 0;
  let mut triples = 0;
  for line in input.lines().map(str::trim) {
    let mut chars = HashMap::new();
    for c in line.chars() {
      *chars.entry(c).or_insert(0) += 1;
    }
    if chars.values().any(|&count| count >= 3) {
      triples += 1;
    }
    if chars.values().any(|&count| count == 2) {
      doubles += 1;
    }
  }
  doubles * triples
}

fn find_common_letters(input: &str) -> String {
  for (a, b) in iproduct!(input.lines(), input.lines()) {
    if a.chars().zip(b.chars()).filter(|(a, b)| a != b).count() == 1 {
      let diff_position = a.chars().zip(b.chars()).position(|(a, b)| a != b).unwrap();
      return format!("{}{}", &a[..diff_position], &a[diff_position + 1..]);
    }
  }
  "".to_owned()
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_input() {
    let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
    let expected = 4 * 3;

    assert_eq!(expected, calculate_cheksum(input));
  }

  #[test]
  fn test_input2() {
    let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
    let expected = "fgij".to_owned();

    assert_eq!(expected, find_common_letters(input));
  }
}
