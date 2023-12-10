use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);
type Map = HashMap<Pos, char>;

pub fn solve(input: &str) -> (u64, u64) {
  let mut map: Map = HashMap::new();
  for (row, line) in input.trim().lines().enumerate() {
    for (col, ch) in line.trim().chars().enumerate() {
      map.insert((row as i32 + 1, col as i32 + 1), ch);
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

  // replace junk pipes ?
  for (p, tile) in map.iter_mut() {
    if !visited.contains(p) {
      *tile = '.';
    }
  }

  // add outer ring ?
  let max = map.keys().max().unwrap().clone();
  for col in 0..=(max.1 + 1) {
    map.insert((0, col), '.');
    map.insert((max.0 + 1, col), '.');
  }
  for row in 0..=(max.0 + 1) {
    map.insert((row, 0), '.');
    map.insert((row, max.1 + 1), '.');
  }

  // flood 0,0 for p2
  let mut next = vec![(0, 0)];
  while let Some(p) = next.pop() {
    map.insert(p, 'O');

    // direct
    if map.get(&up(p)) == Some(&'.') {
      next.push(up(p));
    }
    if map.get(&down(p)) == Some(&'.') {
      next.push(down(p));
    }
    if map.get(&left(p)) == Some(&'.') {
      next.push(left(p));
    }
    if map.get(&right(p)) == Some(&'.') {
      next.push(right(p));
    }

    // corner
    let possible_corners = [
      (up as fn(Pos) -> Pos, 'J', left as fn(Pos) -> Pos, '7'),
      (up, 'J', right, 'F'),
      (down, 'F', left, 'J'),
      (down, '7', right, 'L'),
    ];
    for (v, ve, h, he) in possible_corners {
      if map.get(&v(p)) == Some(&ve)
        && map.get(&h(p)) == Some(&he)
        && map.get(&(v(h(p)))) == Some(&'.')
      {
        next.push(v(h(p)));
      }
    }

    // leaking
    let possible_leaking = [
      (up as fn(Pos) -> Pos, 'J', '|', '7'),
      (up, 'L', '|', 'F'),
      (down, '7', '|', 'J'),
      (down, 'F', '|', 'L'),
      (left, '7', '-', 'F'),
      (left, 'J', '-', 'L'),
      (right, 'F', '-', '7'),
      (right, 'L', '-', 'J'),
    ];

    for (movement, start, cont, end) in possible_leaking {
      if map.get(&movement(p)) == Some(&start) {
        let mut leaking = true;
        let mut p_move = movement(movement(p));
        loop {
          match map.get(&p_move) {
            Some(tile) if *tile == cont => {
              p_move = movement(p_move);
            }
            Some(tile) if *tile == end => {
              p_move = movement(p_move);
              if map.get(&p_move) == Some(&start) {
                p_move = movement(p_move);
              } else {
                break;
              }
            }
            _ => {
              leaking = false;
              break;
            }
          }
        }
        if leaking && map.get(&p_move) == Some(&'.') {
          next.push(p_move);
        }
      }
    }
  }

  let p2 = map.values().filter(|tile| **tile == '.').count();

  dbg_map(&map);

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
  let p = map.iter().find(|(_, ch)| **ch == 'S').unwrap().0.clone();
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

fn dbg_map(map: &Map) {
  let max = map.keys().max().unwrap();
  for row in 1..=max.0 {
    for col in 1..=max.1 {
      match *map.get(&(row, col)).unwrap() {
        'J' => print!("⌟"),
        '7' => print!("⌝"),
        'F' => print!("⌜"),
        ch => print!("{}", ch),
      }
    }
    print!("\n")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
