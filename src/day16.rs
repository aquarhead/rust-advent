use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

use Direction::*;

pub fn solve(input: &str) -> (u64, u64) {
  let mut map = HashMap::new();
  let mut k: Pos = (0, 0);
  for (r, line) in input.trim().lines().enumerate() {
    for (c, tile) in line.char_indices() {
      k = (r as i32, c as i32);
      if tile != '.' {
        map.insert(k.clone(), tile);
      }
    }
  }

  (p1(&map, k, ((0, 0), Right)), p2(&map, k))
}

fn p1(map: &HashMap<Pos, char>, k: Pos, start: (Pos, Direction)) -> u64 {
  let mut beams = vec![start];

  let mut visited = HashSet::new();
  while let Some(b) = beams.pop() {
    if !visited.insert(b.clone()) {
      continue;
    }

    let (p, d) = b;

    match (d, map.get(&p)) {
      (Up, Some('-')) | (Down, Some('-')) => {
        try_add(&mut beams, p, k, Left);
        try_add(&mut beams, p, k, Right);
      }
      (Left, Some('|')) | (Right, Some('|')) => {
        try_add(&mut beams, p, k, Up);
        try_add(&mut beams, p, k, Down);
      }
      (Up, Some('/')) => try_add(&mut beams, p, k, Right),
      (Up, Some('\\')) => try_add(&mut beams, p, k, Left),
      (Down, Some('/')) => try_add(&mut beams, p, k, Left),
      (Down, Some('\\')) => try_add(&mut beams, p, k, Right),
      (Left, Some('/')) => try_add(&mut beams, p, k, Down),
      (Left, Some('\\')) => try_add(&mut beams, p, k, Up),
      (Right, Some('/')) => try_add(&mut beams, p, k, Up),
      (Right, Some('\\')) => try_add(&mut beams, p, k, Down),
      _ => try_add(&mut beams, p, k, d),
    }
  }

  visited
    .into_iter()
    .map(|(p, _)| p)
    .collect::<HashSet<_>>()
    .len() as u64
}

fn p2(map: &HashMap<Pos, char>, k: Pos) -> u64 {
  use rayon::prelude::*;

  let mut starts = Vec::new();
  for r in 0..=k.0 {
    starts.push(((r, 0), Right));
    starts.push(((r, k.1), Left));
  }
  for c in 0..=k.1 {
    starts.push(((0, c), Down));
    starts.push(((k.0, c), Up));
  }

  starts
    .into_par_iter()
    .map(|start| p1(map, k, start))
    .max()
    .unwrap()
}

fn try_add(
  beams: &mut Vec<(Pos, Direction)>,
  (r, c): Pos,
  k: Pos,
  dir: Direction,
) {
  let np = match dir {
    Up => (r - 1, c),
    Down => (r + 1, c),
    Left => (r, c - 1),
    Right => (r, c + 1),
  };
  if (np.0 >= 0) && (np.0 <= k.0) && (np.1 >= 0) && (np.1 <= k.1) {
    beams.push((np, dir));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
    "#;

    assert_eq!((46, 51), solve(input));
  }
}
