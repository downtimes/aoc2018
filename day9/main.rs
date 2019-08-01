use std::collections::VecDeque;

//First entry is player number second is the score
fn calculate(max_marble: usize, num_players: usize) -> (usize, usize) {
  let mut game_field = VecDeque::with_capacity(max_marble);
  game_field.push_back(0);
  let mut score = vec![0; max_marble];
  for (marble_value, player) in (0..num_players).cycle().enumerate().take(max_marble) {
    //Since we added the 0 before to make the pop_front work in all cases.
    let marble_value = marble_value + 1;
    if marble_value % 23 == 0 {
      for _ in 0..7 {
        //unwrap okay since we have at least 7 marbles in the deque.
        let moved = game_field.pop_back().unwrap();
        game_field.push_front(moved);
      }
      let removed_marble = game_field.pop_back().unwrap();
      score[player] += marble_value + removed_marble;
      let first = game_field.pop_front().unwrap();
      game_field.push_back(first);
    } else {
      //unwrap() okay since we always have at least 1 marble in it.
      let first = game_field.pop_front().unwrap();
      game_field.push_back(first);
      game_field.push_back(marble_value);
    }
  }
  
  //Find biggest entry and calculate player number by getting index + 1.
  let val = score.into_iter().enumerate().max_by_key(|&(_, score)| score).unwrap();
  (val.0 + 1, val.1)
}

fn main() {
  let input = include_str!("input1.txt");
  let words = input.split_ascii_whitespace().collect::<Vec<_>>();
  let num_players = words[0].parse::<usize>().unwrap();
  let max_marble = words[6].parse::<usize>().unwrap();

  let (winner1, score1) = calculate(max_marble, num_players);
  let (winner2, score2) = calculate(max_marble * 100, num_players);
  println!("The max score is from player {}: {}", winner1, score1);
  println!("The max score is from player {}: {}", winner2, score2);
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn tes_cases() {
    assert_eq!((5, 32), calculate(25, 9));
    assert_eq!(8317, calculate(1618, 10).1);
    assert_eq!(146_373, calculate(7999, 13).1);
    assert_eq!(2764, calculate(1104, 17).1);
    assert_eq!(54718, calculate(6111, 21).1);
    assert_eq!(37305, calculate(5807, 30).1);
  }
}