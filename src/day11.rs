use std::collections::HashMap;

type Pos = (u64, u64);
type Map = HashMap<Pos, char>;

pub fn solve(input: &str) -> (u64, u64) {
  let map: Map = input
    .trim()
    .lines()
    .enumerate()
    .flat_map(|(row, line)| {
      line.chars().enumerate().filter_map(move |(col, ch)| {
        if ch == '#' {
          Some(((row as u64, col as u64), ch))
        } else {
          None
        }
      })
    })
    .collect();

  (expand(&map, 2), expand(&map, 1000000))
}

fn expand(map: &Map, times: u64) -> u64 {
  let mut poss: Vec<Pos> = map.keys().cloned().collect();
  let mr = poss.iter().map(|p| p.0).max().unwrap();
  let mc = poss.iter().map(|p| p.1).max().unwrap();

  let mut after_row_expand = Vec::new();
  let mut expand = 0;
  for r in 0..=mr {
    let (cur, rest): (Vec<_>, Vec<_>) =
      poss.into_iter().partition(|p| p.0 == r);
    poss = rest;
    if cur.len() > 0 {
      for p in cur {
        after_row_expand.push((p.0 + expand, p.1));
      }
    } else {
      expand += times - 1;
    }
  }

  let mut after_expand = Vec::new();
  let mut expand = 0;
  for c in 0..=mc {
    let (cur, rest): (Vec<_>, Vec<_>) =
      after_row_expand.into_iter().partition(|p| p.1 == c);
    after_row_expand = rest;
    if cur.len() > 0 {
      for p in cur {
        after_expand.push((p.0, p.1 + expand));
      }
    } else {
      expand += times - 1;
    }
  }

  let mut ret = 0;
  let n = after_expand.len();
  for i in 0..(n - 1) {
    for j in (i + 1)..n {
      ret += after_expand[i].0.abs_diff(after_expand[j].0)
        + after_expand[i].1.abs_diff(after_expand[j].1);
    }
  }

  ret
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
    "#;

    assert_eq!(374, solve(input).0);
  }
}
