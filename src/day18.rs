use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);

pub fn solve(input: &str) -> (u64, u64) {
  let mut map = HashMap::new();

  let mut row0 = 0;
  let mut row1 = 0;
  let mut col0 = 0;
  let mut col1 = 0;

  let mut cp = (0, 0);
  map.insert((0, 0), String::new());

  for line in input.trim().lines() {
    let (left, color) = line.split_once(" (#").unwrap();
    let color = color[0..6].to_string();

    let (dir, len) = left.split_once(' ').unwrap();
    let len = len.parse::<i32>().unwrap();
    let (dx, dy) = match dir {
      "U" => (-1, 0),
      "D" => (1, 0),
      "L" => (0, -1),
      "R" => (0, 1),
      _ => panic!("impossible"),
    };
    for _ in 0..len {
      cp = (cp.0 + dx, cp.1 + dy);
      row0 = row0.min(cp.0);
      row1 = row1.max(cp.0);
      col0 = col0.min(cp.1);
      col1 = col1.max(cp.1);
      map.insert(cp, color.clone());
    }
  }

  let mut p1 = 0;
  for c in col0..=col1 {
    let mut inside = (false, false);
    for r in row0..=row1 {
      if (inside.0 && inside.1) || map.contains_key(&(r, c)) {
        p1 += 1;
      }
      if map.contains_key(&(r, c)) {
        if map.contains_key(&(r, c - 1)) {
          inside.0 = !inside.0;
        }
        if map.contains_key(&(r, c + 1)) {
          inside.1 = !inside.1;
        }
      }
    }
  }

  (p1, 0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
    ";

    //     let input = "
    // R 6 (#70c710)
    // D 5 (#0dc571)
    // L 2 (#5713f0)
    // D 2 (#d2c081)
    // R 2 (#59c680)
    // D 3 (#411b91)
    // L 1 (#111111)
    // U 1 (#111111)
    // L 4 (#8ceee2)
    // U 2 (#caa173)
    // L 1 (#1b58a2)
    // U 2 (#caa171)
    // R 2 (#7807d2)
    // U 3 (#a77fa3)
    // L 2 (#015232)
    // U 2 (#7a21e3)
    //     ";

    assert_eq!((62, 0), solve(input));
  }
}
