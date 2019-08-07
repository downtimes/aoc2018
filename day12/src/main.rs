use std::iter;

#[derive(Debug)]
struct Rule {
    before: Vec<bool>,
    after: bool,
}

impl Rule {
    fn matches(&self, other: &[bool]) -> bool {
        self.before == other
    }

    fn parse(line: &str) -> Rule {
        let line = line.trim();
        let parts: Vec<&str> = line.split(" => ").collect();

        Rule {
            before: parts[0].chars().map(to_bool).collect(),
            after: parts[1].chars().map(to_bool).next().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Tape {
    tape: Vec<bool>,
    offset: i64,
    left_increase: bool,
    right_increase: bool,
}

impl Tape {
    fn advance(&self, rules: &[Rule]) -> Tape {
        //Because of the windows we lose the two elements in front and in the back
        //so we chain the iterators with false to restore the elements again.
        //additianally increas the size by 2 for each run.
        let new_tape: Vec<_> = 
            iter::repeat(false).take(if self.left_increase {3} else {2})
            .chain(self.tape.windows(5).map(|part| {
                let mut val = false;
                for rule in rules {
                    if rule.matches(part) {
                        val = rule.after;
                    }
                }
                val
            }))
            .chain(iter::repeat(false).take(if self.right_increase {3} else {2})).collect();

        //if we have not enough room on left size increase in next iteration;
        let left_increase = new_tape[4];
        //if we have not enough room on right size increase in next iteration;
        let right_increase = new_tape[new_tape.len() - 5];
        Tape { 
            tape: new_tape,
            offset: if self.left_increase {self.offset + 1} else {self.offset},
            left_increase,
            right_increase
         }
    }

    fn get_sum(&self) -> i64 {
        let mut sum = 0;
        for (pos, &b) in self.tape.iter().enumerate() {
            if b {
                sum += pos as i64 - self.offset as i64;
            }
        }
        sum
    }
}

fn to_bool(c: char) -> bool {
    c == '#' 
}

//Plants can only move a maximum of 2 fields from one generation to the next
//so we can easily reserve enough space for the field by moving it 2 * number of
//generations to the right before starting the process.
const GENERATIONS: usize = 50_000_000_000;

fn main() {
    let input = include_str!("../input.txt");
    let start: Vec<_> = iter::repeat(false).take(5).chain(input.lines().next().unwrap().trim_start_matches("initial state: ").chars().map(to_bool)).chain(iter::repeat(false).take(5)).collect();
    let mut start_tape = Tape {
        tape: start,
        offset: 5,
        left_increase: false,
        right_increase: false,
    };
    let rules: Vec<_> = input.lines().skip(2).map(|line| Rule::parse(line)).collect();

    let mut old_difference = 0;
    for gen in 0..GENERATIONS {
        if gen == 20 {
            println!("Part1: {}", start_tape.get_sum());
        }

        let new_tape = start_tape.advance(&rules);

        let new_difference = new_tape.get_sum() - start_tape.get_sum();
        //We hope that the difference between tapes stabilizes at some point in time...
        if new_difference == old_difference {
            println!("Part2: {}", new_tape.get_sum() + (GENERATIONS - gen - 1) as i64 * new_difference);
            break;
        }
        start_tape = new_tape;
        old_difference = new_difference;
    }
}
