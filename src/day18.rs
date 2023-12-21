use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);

pub fn solve(input: &str) -> (u64, u64) {
  (p1(input), p2(input))
}

fn p1(input: &str) -> u64 {
  let mut map = HashSet::new();

  let mut row0 = 0;
  let mut row1 = 0;
  let mut col0 = 0;
  let mut col1 = 0;

  let mut cp = (0, 0);
  map.insert((0, 0));

  for line in input.trim().lines() {
    let (left, _) = line.split_once(" (#").unwrap();

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
      map.insert(cp);
    }
  }

  let mut ret = 0;
  for c in col0..=col1 {
    let mut inside = (false, false);
    for r in row0..=row1 {
      if (inside.0 && inside.1) || map.contains(&(r, c)) {
        ret += 1;
      }
      if map.contains(&(r, c)) {
        if map.contains(&(r, c - 1)) {
          inside.0 = !inside.0;
        }
        if map.contains(&(r, c + 1)) {
          inside.1 = !inside.1;
        }
      }
    }
  }

  ret
}

fn p2(input: &str) -> u64 {
  let mut ret = 0;
  let mut hdigs = HashMap::new();

  let mut row0 = 0;
  let mut row1 = 0;
  let mut col0 = 0;
  let mut col1 = 0;

  let mut cp = (0, 0);
  for line in input.trim().lines() {
    let (_, code) = line.split_once(" (#").unwrap();
    let (dist, _) = code[0..5]
      .chars()
      .map(|c| c.to_digit(16).unwrap() as i32)
      .rev()
      .fold((0, 1), |(acc, mul), x| (acc + x * mul, mul * 16));

    ret += dist as u64;

    match code[5..6].as_ref() {
      "3" => {
        cp.0 -= dist;
      }
      "1" => {
        cp.0 += dist;
      }
      "2" => {
        let nc = cp.1 - dist;
        hdigs.entry(cp.0).or_insert(Vec::new()).push((nc, cp.1));
        cp.1 = nc;
      }
      "0" => {
        let nc = cp.1 + dist;
        hdigs.entry(cp.0).or_insert(Vec::new()).push((cp.1, nc));
        cp.1 = nc;
      }
      _ => panic!("impossible"),
    };

    row0 = row0.min(cp.0);
    row1 = row1.max(cp.0);
    col0 = col0.min(cp.1);
    col1 = col1.max(cp.1);
  }

  let mut hdigs = hdigs.into_iter().collect::<Vec<_>>();
  hdigs.sort_by_key(|(r, _)| *r);

  let mut line = vec![(false, false); (col1 - col0 + 1) as usize];

  let mut r = row0 - 1;
  for (nr, digs) in hdigs.into_iter() {
    ret +=
      line.iter().filter(|x| x.0 && x.1).count() as u64 * ((nr - r - 1) as u64);

    let mut new_line = line.clone();

    for (left, right) in digs {
      {
        let i = (left - col0) as usize;
        new_line[i].1 = !line[i].1;
      }
      {
        let i = (right - col0) as usize;
        new_line[i].0 = !line[i].0;
      }
      for n in left + 1..right {
        let i = (n - col0) as usize;
        new_line[i].0 = !line[i].0;
        new_line[i].1 = !line[i].1;
      }
    }

    ret += line
      .iter()
      .zip(new_line.iter())
      .filter(|(x, y)| x.0 && x.1 && y.0 && y.1)
      .count() as u64;
    line = new_line;

    r = nr;
  }

  ret
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

    assert_eq!((62, 952408144115), solve(input));
  }
}
