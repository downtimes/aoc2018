use std::collections::HashMap;
use itertools::Itertools;

fn main() {
  let input = std::fs::read_to_string("input1.txt").unwrap();
  println!("{}", calculate_checksum(&input));
  println!("{}", find_common_letters(&input));
}

fn calculate_checksum(input: &str) -> u32 {
  let mut doubles = 0;
  let mut triples = 0;
  for line in input.lines().map(str::trim) {
    //NOTE: Since our input is ASCII only it also would be feasable to only use
    //      an array with the 26 possible char values as index into it. For 
    //      ASCII input this method is much faster. With a hashmap we have a more
    //      general solution.
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
  //NOTE: Not the most performant code. The used algorithm is  
  //      generally an O(n^2) algorithm. Since the input is really small we don't
  //      worry about it here.
  for (a, b) in input.lines().map(str::trim).tuple_combinations() {
    if a.chars().zip(b.chars()).filter(|(a, b)| a != b).count() == 1 {
      return a
        .chars()
        .zip(b.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect();
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

    assert_eq!(expected, calculate_checksum(input));
  }

  #[test]
  fn test_input2() {
    let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
    let expected = "fgij".to_owned();

    assert_eq!(expected, find_common_letters(input));
  }
}
