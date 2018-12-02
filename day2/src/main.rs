use std::collections::HashMap;
use itertools::iproduct;

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
    if chars.values().any(|&count| count >= 3) { triples += 1; }
    if chars.values().any(|&count| count == 2) { doubles += 1; }
  }
  doubles * triples
}

fn find_common_letters(input: &str) -> String {
  for (a, b) in iproduct!(input.lines(), input.lines()) {
    if a.chars().zip(b.chars()).filter(|(a, b)| a != b).count() == 1 {
      let diff_position = a.chars().zip(b.chars()).position(|(a, b)| a != b).unwrap();
      let mut result = a[..diff_position].to_owned();
      result.push_str(&a[diff_position + 1..]);
      return result;
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
