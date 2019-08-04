fn calculate_power(pos: (usize, usize), serial_number: usize) -> i64 {
    let rack_id = pos.0 + 10;
    let power_level = (rack_id * pos.1 + serial_number) * rack_id;
    let hundredth = (power_level / 100) % 10;
    hundredth as i64 - 5
}

const MAX_GRID: usize = 300;

fn get_best_for_square(field: &[Vec<i64>], square_size: usize) -> (i64, usize, usize, usize) {
    let mut results = Vec::new();
    for x in 1..=MAX_GRID - square_size {
        for y in 1..=MAX_GRID - square_size {
            let value = field[x + square_size - 1][y + square_size - 1] 
                        + field[x - 1][y - 1]
                        - field[x - 1][y + square_size - 1]
                        - field[x + square_size - 1][y - 1];
            let entry = (value, x, y, square_size);
            results.push(entry);
        }
    }
    
    results.into_iter().max().unwrap()
}

fn main() {
    let serial_number = 9810;
    let mut power_levels = vec![vec![0; MAX_GRID + 1]; MAX_GRID + 1];
    //Precalculate the power of all sums from the top left corner to the current (x, y)
    //For convenience we also increase the board size by the 0 row and 0 column.
    for x in 0..=MAX_GRID {
        for y in 0..=MAX_GRID {
            if x == 0 || y == 0 { 
                power_levels[x][y] = 0;
            } else {
                let power_level = calculate_power((x, y), serial_number);
                power_levels[x][y] = power_level 
                                    + power_levels[x][y - 1] 
                                    + power_levels[x - 1][y] 
                                    - power_levels[x - 1][y - 1];
            }
        }
    }

    let max1 = get_best_for_square(&power_levels, 3);
    println!("Result part1 is: ({}, {}) score: {}", max1.1, max1.2, max1.0);

    let max_overall = (2..MAX_GRID).map(|square_size| get_best_for_square(&power_levels, square_size)).max().unwrap();
    println!("Result part2 is: ({}, {}, {}) score: {}", max_overall.1, max_overall.2, max_overall.3, max_overall.0);
}
