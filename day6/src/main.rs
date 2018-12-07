use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let parsed = parse_input(&input);
    println!("{}", largest_area(parsed.clone()));
    println!("{}", safe_area_size(parsed, 10000));
}

fn manhatten_distance(p1: (i32, i32), p2: (i32, i32)) -> u32 {
    let x = (p1.0 - p2.0).abs() as u32;
    let y = (p1.1 - p2.1).abs() as u32;
    x + y
}

#[derive(Clone)]
struct Coordinate {
    id: usize,
    x: i32,
    y: i32,
    working_set: HashSet<(i32, i32)>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum State {
    Claimed(usize),
    Empty,
    Collision,
}

fn parse_input(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let point: (i32, i32) = line
                .split(',')
                .map(str::trim)
                .map(|num| num.parse().unwrap())
                .collect_tuple()
                .expect("We only support tuples of 2 items");
            let mut coordinate = Coordinate {
                id: id,
                x: point.0,
                y: point.1,
                working_set: HashSet::new(),
            };
            coordinate.working_set.insert(point);
            coordinate
        })
        .collect()
}

fn safe_area_size(coordinates: Vec<Coordinate>, max_manhatten: u32) -> u32 {
    let min_x = coordinates.iter().min_by_key(|c| c.x).unwrap().x;
    let max_x = coordinates.iter().max_by_key(|c| c.x).unwrap().x;
    let min_y = coordinates.iter().min_by_key(|c| c.y).unwrap().y;
    let max_y = coordinates.iter().max_by_key(|c| c.y).unwrap().y;

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut map = vec![State::Empty; (width * height) as usize];
    let map_index = |x: i32, y: i32| {
        //We only expect a positive number playing field.
        assert!(x >= 0 && y >= 0);
        ((y - min_y) * width + (x - min_x)) as usize
    };

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let combined_distance: u32 = coordinates
                .iter()
                .map(|coord| manhatten_distance((coord.x, coord.y), (x, y)))
                .sum();
            if combined_distance < max_manhatten {
                map[map_index(x, y)] = State::Collision;
            } else {
                map[map_index(x, y)] = State::Empty;
            }
        }
    }

    map.iter()
        .filter(|&&state| state == State::Collision)
        .count() as u32
}

//TODO: Rewrite in the sane version where we visit every point in the grid only
//      once
//Crazy function that is much slower than the easier approach...
fn largest_area(coordinates: Vec<Coordinate>) -> u32 {
    let mut coordinates = coordinates;
    let min_x = coordinates.iter().min_by_key(|c| c.x).unwrap().x;
    let max_x = coordinates.iter().max_by_key(|c| c.x).unwrap().x;
    let min_y = coordinates.iter().min_by_key(|c| c.y).unwrap().y;
    let max_y = coordinates.iter().max_by_key(|c| c.y).unwrap().y;

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut map = vec![State::Empty; (width * height) as usize];
    let map_index = |x: i32, y: i32| {
        //We only expect a positive number playing field.
        assert!(x >= 0 && y >= 0);
        ((y - min_y) * width + (x - min_x)) as usize
    };
    let inside_check = |x: i32, y: i32| x >= min_x && x <= max_x && y >= min_y && y <= max_y;
    while coordinates.iter().any(|c| !c.working_set.is_empty()) {
        //Claim the current working sets as our own
        for coord in &coordinates {
            for p in &coord.working_set {
                if map[map_index(p.0, p.1)] == State::Empty {
                    map[map_index(p.0, p.1)] = State::Claimed(coord.id);
                }
            }
        }

        //Extend the radius of each coordinate by one.
        for coord in &mut coordinates {
            let mut new_set = HashSet::new();
            for p in &coord.working_set {
                let neighbours: HashSet<(i32, i32)> = get_neighbours(p.0, p.1)
                    .into_iter()
                    .cloned()
                    .filter(|(x, y)| inside_check(*x, *y) && map[map_index(*x, *y)] == State::Empty)
                    .collect();
                new_set = &new_set | &neighbours;
            }
            coord.working_set = new_set;
        }

        //Check for collisions.
        let mut collisions = HashSet::new();
        for (coorda, coordb) in coordinates.iter().cartesian_product(coordinates.iter()) {
            if coorda.id == coordb.id {
                continue;
            }
            let local_collisions = &coorda.working_set & &coordb.working_set;
            collisions = &collisions | &local_collisions;
        }

        //Remove all collisions from the board.
        for collision in collisions.iter() {
            map[map_index(collision.0, collision.1)] = State::Collision;
        }
    }

    // Get a set of all ids that are at the border and therefore endless.
    let mut endless = HashSet::new();
    //left and right borders
    for x in [min_x, max_x].iter() {
        for y in min_y..=max_y {
            match map[map_index(*x, y)] {
                State::Claimed(id) => {
                    endless.insert(id);
                }
                _ => {}
            }
        }
    }
    //top and bottom borders
    for y in [min_y, max_y].iter() {
        for x in min_x..=max_x {
            match map[map_index(x, *y)] {
                State::Claimed(id) => {
                    endless.insert(id);
                }
                _ => {}
            }
        }
    }
    //calculate the amount for each id
    let mut count_map = HashMap::new();
    for state in map.iter() {
        match state {
            State::Claimed(id) => {
                *count_map.entry(id).or_insert(0) += 1;
            }
            _ => {}
        }
    }

    *count_map
        .iter()
        .filter(|(id, _)| !endless.contains(id))
        .map(|(_, c)| c)
        .max()
        .unwrap()
}

fn get_neighbours(x: i32, y: i32) -> [(i32, i32); 4] {
    [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_input() {
        let input = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";
        let expected = 17;

        let parsed = parse_input(input);
        assert_eq!(expected, largest_area(parsed));
    }
    #[test]
    fn tes_manhatten() {
        assert_eq!(2, manhatten_distance((1, 1), (2, 2)));
        assert_eq!(5, manhatten_distance((4, 1), (0, 0)));
        assert_eq!(5, manhatten_distance((0, 0), (1, 4)));
    }
    #[test]
    fn test_input2() {
        let input = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";
        let expected = 16;

        let parsed = parse_input(input);
        assert_eq!(expected, safe_area_size(parsed, 32));
    }
}
