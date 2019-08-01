use regex::Regex;

#[derive(Debug)]
struct Light {
    position: (i32, i32),
    velocity: (i32, i32)
}

impl Light {
    fn is_neighbour(&self, other_light: &Light) -> bool {
        let positions = vec!(-1, 0, 1, 0, -1, -1, 1, 1).iter()
                   .zip(vec!(0, -1, 0, 1, 1, -1, 1, -1).iter())
                   .map(|(xo, yo)| (self.position.0 + xo, self.position.1 + yo))
                   .collect::<Vec<_>>();
        positions.iter().any(|&x| x == other_light.position)
    }
}

fn parse_light(line: &str) -> Light {
    let re = Regex::new(r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>").unwrap();
    let values = re.captures(line).unwrap();
    Light {
        position: (values[1].parse().unwrap(), values[2].parse().unwrap()),
        velocity: (values[3].parse().unwrap(), values[4].parse().unwrap()),
    }
}

//TODO(MA): Find a better metric for when we probably have a solution.
//          this function only works when ALL the lights participate in the message
//          Some kind of "clusterness" of the lights would probably be more robust.
//          @see https://www.reddit.com/r/adventofcode/comments/a73b86/2018_day_10_yes_you_solved_it_but/
fn lights_aligned(lights: &[Light]) -> bool {
    //As long as the lights are to far away we don't have to check for a message.
    let min_y = lights.iter().map(|light| light.position.1).min().unwrap_or(0);
    let max_y = lights.iter().map(|light| light.position.1).max().unwrap_or(0);
    if (max_y - min_y) > 50 { return false; }

    lights.iter().all(|light|
        lights.iter().any(|olight| light.is_neighbour(olight))
    )
}


fn move_lights_one_second(lights: &mut[Light]) {
    for light in lights.iter_mut() {
        light.position.0 += light.velocity.0;
        light.position.1 += light.velocity.1;
    }
}

fn format_message(lights: &[Light]) -> String {
    let min_x = lights.iter().map(|light| light.position.0).min().unwrap_or(0);
    let max_x = lights.iter().map(|light| light.position.0).max().unwrap_or(0);
    let min_y = lights.iter().map(|light| light.position.1).min().unwrap_or(0);
    let max_y = lights.iter().map(|light| light.position.1).max().unwrap_or(0);
    let size = (max_x - min_x) as usize * (max_y - min_y) as usize;
    let mut result = String::with_capacity(size);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if lights.iter().any(|light| light.position == (x, y)) {
                result.push('*');
            } else {
                result.push('.');
            }
        }
        result.push('\n');
    }
    result
}

fn main() {
    let input = include_str!("../input.txt");
    let mut lights = input.lines().map(|line| parse_light(line)).collect::<Vec<_>>();
    let mut second = 0;
    while !lights_aligned(&lights) { 
        move_lights_one_second(&mut lights);
        second += 1;
    }
    println!("{}", format_message(&lights));
    println!("Message appeared after {} seconds.", second);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_neighbour() {
        let l1 = Light {
            position: (0, 0),
            velocity: (0, 0),
        };
        let l2 = Light {
            position: (1, 0),
            velocity: (0, 0),
        };
        assert!(l1.is_neighbour(&l2));
    }
}