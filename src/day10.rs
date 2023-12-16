use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);
type Map = HashMap<Pos, char>;

pub fn solve(input: &str) -> (u64, u64) {
  let mut map: Map = HashMap::new();
  for (row, line) in input.trim().lines().enumerate() {
    for (col, ch) in line.trim().chars().enumerate() {
      map.insert((row as i32, col as i32), ch);
    }
  }

  let start_pos = replace_start(&mut map);

  // walk p1
  let mut visited = HashSet::new();
  let mut visit = vec![start_pos];
  let mut p1 = 0;

  while let Some(p) = visit.pop() {
    match map.get(&p).unwrap() {
      '|' => {
        if !visited.contains(&up(p)) {
          visit.push(up(p));
        }
        if !visited.contains(&down(p)) {
          visit.push(down(p));
        }
      }
      '-' => {
        if !visited.contains(&left(p)) {
          visit.push(left(p));
        }
        if !visited.contains(&right(p)) {
          visit.push(right(p));
        }
      }
      'L' => {
        if !visited.contains(&up(p)) {
          visit.push(up(p));
        }
        if !visited.contains(&right(p)) {
          visit.push(right(p));
        }
      }
      'J' => {
        if !visited.contains(&up(p)) {
          visit.push(up(p));
        }
        if !visited.contains(&left(p)) {
          visit.push(left(p));
        }
      }
      '7' => {
        if !visited.contains(&left(p)) {
          visit.push(left(p));
        }
        if !visited.contains(&down(p)) {
          visit.push(down(p));
        }
      }
      'F' => {
        if !visited.contains(&right(p)) {
          visit.push(right(p));
        }
        if !visited.contains(&down(p)) {
          visit.push(down(p));
        }
      }
      _ => panic!("impossible"),
    }

    visited.insert(p);
    p1 += 1;
  }

  // p2, first replace junk pipes
  for (p, tile) in map.iter_mut() {
    if !visited.contains(p) {
      *tile = '.';
    }
  }

  let max = *map.keys().max().unwrap();

  let mut sub_visited = HashSet::new();
  let mut visit = vec![(0, 0)];
  while let Some(sp) = visit.pop() {
    let mut try_visit = Vec::new();
    match (sp.0 % 2, sp.1 % 2) {
      (0, 0) => {
        // top left
        if sp.0 > 0 {
          try_visit.push((sp.0 - 1, sp.1));
        }
        if sp.1 > 0 {
          try_visit.push((sp.0, sp.1 - 1));
        }
        match *map.get(&(sp.0 / 2, sp.1 / 2)).unwrap() {
          'J' => {}
          'L' | '|' => try_visit.push((sp.0 + 1, sp.1)),
          '7' | '-' => try_visit.push((sp.0, sp.1 + 1)),
          _ => {
            try_visit.push((sp.0 + 1, sp.1));
            try_visit.push((sp.0, sp.1 + 1));
          }
        }
      }
      (0, 1) => {
        // top right
        if sp.0 > 0 {
          try_visit.push((sp.0 - 1, sp.1));
        }
        if sp.1 < max.1 * 2 {
          try_visit.push((sp.0, sp.1 + 1));
        }
        match *map.get(&(sp.0 / 2, sp.1 / 2)).unwrap() {
          'L' => {}
          'J' | '|' => try_visit.push((sp.0 + 1, sp.1)),
          'F' | '-' => try_visit.push((sp.0, sp.1 - 1)),
          _ => {
            try_visit.push((sp.0 + 1, sp.1));
            try_visit.push((sp.0, sp.1 - 1));
          }
        }
      }
      (1, 0) => {
        // bot left
        if sp.0 < max.0 * 2 {
          try_visit.push((sp.0 + 1, sp.1));
        }
        if sp.1 > 0 {
          try_visit.push((sp.0, sp.1 - 1));
        }
        match *map.get(&(sp.0 / 2, sp.1 / 2)).unwrap() {
          '7' => {}
          'J' | '-' => try_visit.push((sp.0, sp.1 + 1)),
          'F' | '|' => try_visit.push((sp.0 - 1, sp.1)),
          _ => {
            try_visit.push((sp.0, sp.1 + 1));
            try_visit.push((sp.0 - 1, sp.1));
          }
        }
      }
      (1, 1) => {
        //bot right
        if sp.0 < max.0 * 2 {
          try_visit.push((sp.0 + 1, sp.1));
        }
        if sp.1 < max.0 * 2 {
          try_visit.push((sp.0, sp.1 + 1));
        }
        match *map.get(&(sp.0 / 2, sp.1 / 2)).unwrap() {
          'F' => {}
          'L' | '-' => try_visit.push((sp.0, sp.1 - 1)),
          '7' | '|' => try_visit.push((sp.0 - 1, sp.1)),
          _ => {
            try_visit.push((sp.0, sp.1 - 1));
            try_visit.push((sp.0 - 1, sp.1));
          }
        }
      }
      _ => panic!("impossible"),
    };

    visit.extend(try_visit.into_iter().filter(|p| !sub_visited.contains(p)));

    sub_visited.insert(sp);
  }

  let p2 = map
    .into_iter()
    .filter(|(p, t)| {
      *t == '.'
        && [
          (p.0 * 2, p.1 * 2),
          (p.0 * 2, p.1 * 2 + 1),
          (p.0 * 2 + 1, p.1 * 2),
          (p.0 * 2 + 1, p.1 * 2 + 1),
        ]
        .iter()
        .any(|sp| !sub_visited.contains(sp))
    })
    .count();

  (p1 / 2, p2 as u64)
}

fn up(p: Pos) -> Pos {
  (p.0 - 1, p.1)
}

fn down(p: Pos) -> Pos {
  (p.0 + 1, p.1)
}

fn left(p: Pos) -> Pos {
  (p.0, p.1 - 1)
}

fn right(p: Pos) -> Pos {
  (p.0, p.1 + 1)
}

fn replace_start(map: &mut Map) -> Pos {
  let p = *map.iter().find(|(_, ch)| **ch == 'S').unwrap().0;
  let mut connected = (false, false, false, false);
  match map.get(&up(p)) {
    Some('|') => connected.0 = true,
    Some('7') => connected.0 = true,
    Some('F') => connected.0 = true,
    _ => {}
  }
  match map.get(&down(p)) {
    Some('|') => connected.1 = true,
    Some('L') => connected.1 = true,
    Some('J') => connected.1 = true,
    _ => {}
  }
  match map.get(&left(p)) {
    Some('-') => connected.2 = true,
    Some('L') => connected.2 = true,
    Some('F') => connected.2 = true,
    _ => {}
  }
  match map.get(&right(p)) {
    Some('-') => connected.3 = true,
    Some('J') => connected.3 = true,
    Some('7') => connected.3 = true,
    _ => {}
  }

  match connected {
    (true, true, false, false) => map.insert(p, '|'),
    (true, false, true, false) => map.insert(p, 'J'),
    (true, false, false, true) => map.insert(p, 'L'),
    (false, true, true, false) => map.insert(p, '7'),
    (false, true, false, true) => map.insert(p, 'F'),
    (false, false, true, true) => map.insert(p, '-'),
    _ => panic!("impossible"),
  };

  p
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let input = r#"
S7
LJ
    "#;

    assert_eq!((2, 0), solve(input));
  }

  #[test]
  fn test2() {
    let input = r#"
S-7
|.|
L-J
    "#;

    assert_eq!((4, 1), solve(input));
  }

  #[test]
  fn test_example() {
    let input = r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
    "#;

    assert_eq!((8, 1), solve(input));
  }

  #[test]
  fn test_example2() {
    let input = r#"
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
    "#;

    assert_eq!((22, 4), solve(input));
  }

  #[test]
  fn test_example3() {
    let input = r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
    "#;

    assert_eq!((70, 8), solve(input));
  }

  #[test]
  fn test_example4() {
    let input = r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
    "#;

    assert_eq!((80, 10), solve(input));
  }
}
