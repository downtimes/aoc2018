use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn manhatten_distance(p1: (i32, i32), p2: (i32, i32)) -> u32 {
    let x = (p1.0 - p2.0).abs() as u32;
    let y = (p1.1 - p2.1).abs() as u32;
    x + y
}

struct Coordinate {
    id: usize,
    x: i32,
    y: i32,
    working_set: HashSet<(i32, i32)>,
}

#[derive(Copy, Clone)]
enum State {
    Claimed(usize),
    Empty,
    Collision,
}

fn largest_area(input: &str) -> u32 {
    let mut coordinates = input
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
        .collect::<Vec<Coordinate>>();
    let min_x = coordinates.iter().min_by_key(|c| c.x).unwrap().x;
    let max_x = coordinates.iter().max_by_key(|c| c.x).unwrap().x;
    let min_y = coordinates.iter().min_by_key(|c| c.y).unwrap().y;
    let max_y = coordinates.iter().max_by_key(|c| c.y).unwrap().y;

    //min and max is our playfield.
    //we have a set and id per point.
    //we have a playfield to keep track of who owns what.
    //each point has itself in the starting set.
    //each iteration: for each point for each point in their set if it the point is inside the playing field and not claimed or collided.
    //                  -Put all neighbours in the set
    //                  -mark the point as our own in the playing field.
    // after this is done for all points do a set intersection between all of them
    // and remove points of collision and mark in playing field as not reachable
    // Finished, when all sets are empty.
    let width = max_x - min_x;
    let height = max_y - min_y;
    let map = vec![State::Empty; (width * height) as usize];
    let map_index = |x: i32, y: i32| {
        ((y - min_y) * width + (x - min_x)) as usize
    };
    while coordinates.iter().any(|c| !c.working_set.is_empty()) {

    }
    0
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_input() {
        let input = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";
        let expected = 17;

        assert_eq!(expected, largest_area(input));
    }
    #[test]
    fn tes_manhatten() {
        assert_eq!(2, manhatten_distance((1, 1), (2, 2)));
        assert_eq!(5, manhatten_distance((4, 1), (0, 0)));
        assert_eq!(5, manhatten_distance((0, 0), (1, 4)));
    }
}
