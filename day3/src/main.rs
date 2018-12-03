use itertools::iproduct;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    println!("{}", overlapping_points(&parse_claims(&input)));
    println!("{}", non_overlapping_claim_id(&parse_claims(&input)));
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Claim {
    id: u32,
    topleft: Point,
    width: u32,
    height: u32,
}

fn parse_claims(input: &str) -> Vec<Claim> {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            let line_items: Vec<_> = line
                .split(|c| c == '@' || c == ':')
                .map(str::trim)
                .collect();
            let id = line_items[0][1..].parse().unwrap();
            let mut point_stuff = line_items[1].split(',');
            let mut extend = line_items[2].split('x');
            Claim {
                id,
                topleft: Point {
                    x: point_stuff.next().unwrap().parse().unwrap(),
                    y: point_stuff.next().unwrap().parse().unwrap(),
                },
                width: extend.next().unwrap().parse().unwrap(),
                height: extend.next().unwrap().parse().unwrap(),
            }
        }).collect()
}

fn overlapping_points(claims: &[Claim]) -> u32 {
    let mut cloth_map = HashMap::new();
    for claim in claims {
        for (x, y) in iproduct!(
            claim.topleft.x..claim.topleft.x + claim.width,
            claim.topleft.y..claim.topleft.y + claim.height
        ) {
            *cloth_map.entry((x, y)).or_insert(0) += 1;
        }
    }

    cloth_map.values().filter(|&v| v > &1).count() as u32
}

fn non_overlapping_claim_id(claims: &[Claim]) -> u32 {
    let mut cloth_map = HashMap::new();
    let mut candidates = HashSet::new();
    for claim in claims {
        candidates.insert(claim.id);
        for (x, y) in iproduct!(
            claim.topleft.x..claim.topleft.x + claim.width,
            claim.topleft.y..claim.topleft.y + claim.height
        ) {
            match cloth_map.get(&(x, y)) {
                Some(id) => {
                    candidates.remove(id);
                    candidates.remove(&claim.id);
                }
                None => {
                    cloth_map.insert((x, y), claim.id);
                }
            }
        }
    }
    candidates.into_iter().next().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        let expected = 4;

        assert_eq!(expected, overlapping_points(&parse_claims(input)));
    }

    #[test]
    fn test_input2() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        let expected = 3;

        assert_eq!(expected, non_overlapping_claim_id(&parse_claims(input)));
    }
}
