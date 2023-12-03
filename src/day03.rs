use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);

pub fn solve(input: String) -> u32 {
  let mut map = HashMap::<Pos, char>::new();
  let mut symbol_pos = Vec::<Pos>::new();

  for (x, line) in input.trim().lines().enumerate() {
    for (y, char) in line.trim().char_indices() {
      let pos: Pos = (x as i32, y as i32);
      match char {
        '.' => {}
        x if x >= '0' && x <= '9' => {
          let _ = map.insert(pos, char);
        }
        _symbol => symbol_pos.push(pos),
      }
    }
  }

  let mut p1: u32 = 0;
  let mut num_marked = HashSet::<Pos>::new();
  for sp in symbol_pos {
    for p in surround(sp) {
      if num_marked.contains(&p) {
        continue;
      }
      num_marked.insert(p);

      if let Some(n) = map.get(&p) {
        let mut num_str = String::from(*n);
        let mut p_front = (p.0, p.1 - 1);
        while let Some(n) = map.get(&p_front) {
          num_marked.insert(p_front);
          num_str.insert(0, *n);
          p_front.1 -= 1;
        }

        let mut p_end = (p.0, p.1 + 1);
        while let Some(n) = map.get(&p_end) {
          num_marked.insert(p_end);
          num_str.push(*n);
          p_end.1 += 1;
        }

        p1 += num_str.parse::<u32>().unwrap();
      }
    }
  }

  p1
}

fn surround(p: Pos) -> Vec<Pos> {
  vec![
    (p.0 - 1, p.1 - 1),
    (p.0 - 1, p.1),
    (p.0 - 1, p.1 + 1),
    (p.0, p.1 - 1),
    (p.0, p.1 + 1),
    (p.0 + 1, p.1 - 1),
    (p.0 + 1, p.1),
    (p.0 + 1, p.1 + 1),
  ]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let input = r#"
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    "#;

    assert_eq!(4361, solve(input.to_string()));
  }
}
