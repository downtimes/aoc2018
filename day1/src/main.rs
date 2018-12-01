use std::collections::HashSet;
use std::str::FromStr;

fn change_frequency(start: i32, input: &str) -> i32 {
    input
        .split(',')
        .map(str::trim)
        .filter_map(|s| i32::from_str(s).ok())
        .fold(start, |current, num| current + num)
}

fn find_first_double_frequency(input: &str) -> i32 {
    let inputs = input
        .split(',')
        .map(str::trim)
        .filter_map(|s| i32::from_str(s).ok());
    let mut current = 0;
    let mut previously_found = HashSet::new();
    for next in inputs.cycle() {
        if !previously_found.insert(current) {
            break;
        }
        current += next;
    }
    current
}

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let input = input.lines().collect::<Vec<_>>();
    let input = input.join(",");
    let result = change_frequency(0, &input);
    println!("{}", result);
    let first_double = find_first_double_frequency(&input);
    println!("{}", first_double);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_input() {
        let input1 = "+1, -2, +3, +1";
        let expected1 = 3;
        let input2 = "+1, +1, +1";
        let expected2 = 3;
        let input3 = "+1, +1, -2";
        let expected3 = 0;
        let input4 = "-1, -2, -3";
        let expected4 = -6;

        assert_eq!(expected1, change_frequency(0, input1));
        assert_eq!(expected2, change_frequency(0, input2));
        assert_eq!(expected3, change_frequency(0, input3));
        assert_eq!(expected4, change_frequency(0, input4));
    }

    #[test]
    fn test_input2() {
        let input1 = "+1, -2, +3, +1";
        let expected1 = 2;
        let input2 = "+1, -1";
        let expected2 = 0;
        let input3 = "+3, +3, +4, -2, -4";
        let expected3 = 10;
        let input4 = "-6, +3, +8, +5, -6";
        let expected4 = 5;
        let input5 = "+7, +7, -2, -7, -4";
        let expected5 = 14;

        assert_eq!(expected1, find_first_double_frequency(input1));
        assert_eq!(expected2, find_first_double_frequency(input2));
        assert_eq!(expected3, find_first_double_frequency(input3));
        assert_eq!(expected4, find_first_double_frequency(input4));
        assert_eq!(expected5, find_first_double_frequency(input5));
    }
}
