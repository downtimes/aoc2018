fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    println!("{}", reduce(&input, None).len());
    println!("{}", find_shortest_reduction(&input));
}

fn to_delete(a: char, b: char) -> bool {
    a != b && a.to_ascii_uppercase() == b.to_ascii_uppercase()
}

fn find_shortest_reduction(input: &str) -> u32 {
    (b'a'..=b'z').map(|c| reduce(input, Some(c as char)).len()).min().unwrap() as u32
}

fn reduce(input: &str, skip_char: Option<char>) -> String {
    let mut old = input.to_owned();
    loop {
        let new = input
            .chars()
            .fold(String::new(), |mut acc, c| match acc.chars().last() {
                _ if skip_char == Some(c.to_ascii_lowercase()) => acc,
                Some(lc) if to_delete(lc, c) => {
                    acc.pop();
                    acc
                }
                _ => {
                    acc.push(c);
                    acc
                }
            });
        if new.len() == old.len() {
            break;
        } else {
            old = new;
        }
    }
    old
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = "dabAcCaCBAcCcaDA";
        let expected = "dabCBAcaDA";

        assert_eq!(expected, reduce(input, None));
    }

    #[test]
    fn test_input2() {
        let input = "dabAcCaCBAcCcaDA";
        let expected = 4;

        assert_eq!(expected, find_shortest_reduction(input));
    }
}
