use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let parsed = parse_input(&input);
    println!("{}", construction_order(construct_map(&parsed)));
    println!("{}", time_required(construct_map(&parsed), 5, 60));
}

fn parse_input(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .skip(1)
                .filter(|c| c.is_ascii_uppercase())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn construct_map(instructions: &[(char, char)]) -> HashMap<char, HashSet<char>> {
    let mut construction_map = HashMap::<char, HashSet<char>>::new();
    for &(before, after) in instructions {
        construction_map.entry(after).or_default().insert(before);
    }

    //Insert items with no dependencies into the map
    for (before, _) in instructions {
        if !construction_map.contains_key(before) {
            construction_map.insert(*before, HashSet::new());
        }
    }
    construction_map
}

fn construction_order(construction_map: HashMap<char, HashSet<char>>) -> String {
    let mut construction_map = construction_map;
    let mut result = String::new();
    while !construction_map.is_empty() {
        let candidate = construction_map
            .iter()
            .filter(|(_, depends)| depends.is_empty())
            .map(|(c, _)| c)
            .cloned()
            .min()
            .unwrap();
        construction_map.remove(&candidate);
        result.push(candidate);

        for entry in &mut construction_map {
            entry.1.remove(&candidate);
        }
    }

    result
}

#[derive(Copy, Clone)]
struct Worker {
    item: Option<char>,
    busy_time: u32,
}

fn time_required(
    construction_map: HashMap<char, HashSet<char>>,
    workers: u8,
    time_base: u32,
) -> u32 {
    let mut construction_map = construction_map;
    let mut workers = vec![
        Worker {
            item: None,
            busy_time: 0,
        };
        workers as usize
    ];
    let mut current_second = 0;
    while !construction_map.is_empty() || workers.iter().any(|w| w.busy_time > 0) {
        //Adjust workers time and check for any items that finished
        for worker in &mut workers {
            if worker.busy_time > 0 {
                worker.busy_time -= 1;
            }

            //The worker finished an item remove it from all dependencies
            if worker.busy_time == 0 {
                if let Some(c) = worker.item {
                    for entry in &mut construction_map {
                        entry.1.remove(&c);
                    }
                    worker.item = None;
                }
            }
        }

        let mut candidates: Vec<_> = construction_map
            .iter()
            .filter(|(_, depends)| depends.is_empty())
            .map(|(c, _)| c)
            .cloned()
            .collect();
        candidates.sort();

        for cand in candidates {
            for worker in &mut workers {
                if worker.item == None {
                    worker.item = Some(cand);
                    worker.busy_time = time_base + (cand as u32 - 65 + 1);
                    construction_map.remove(&cand);
                    break;
                }
            }
        }

        current_second += 1;
    }
    current_second - 1
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_input() {
        let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        let expected = "CABDFE".to_owned();

        let parsed = parse_input(input);
        assert_eq!(expected, construction_order(construct_map(&parsed)));
    }

    #[test]
    fn test_input2() {
        let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        let expected = 15;

        let parsed = parse_input(input);
        assert_eq!(expected, time_required(construct_map(&parsed), 2, 0));
    }
}
