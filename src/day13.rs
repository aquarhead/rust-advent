use rayon::prelude::*;

pub fn solve(input: &str) -> (u64, u64) {
  let ps = input
    .trim()
    .split("\n\n")
    .map(|x| x.to_string())
    .collect::<Vec<_>>();

  ps.into_par_iter()
    .map(|p| {
      let mut rows = Vec::new();
      let mut cols = Vec::new();
      for line in p.trim().lines() {
        // first line
        if cols.len() == 0 {
          cols = vec![0; line.len()];
        }
        rows.push(line.chars().map(digit).fold(0, |acc, x| (acc << 1) + x));
        for (i, x) in line.char_indices() {
          cols[i] <<= 1;
          cols[i] += digit(x);
        }
      }

      let p1 = p1(&rows, &cols);

      (p1, p2(rows, cols, p1))
    })
    .reduce(|| (0, 0), |acc, res| (acc.0 + res.0, acc.1 + res.1))
}

fn p1(rows: &[u64], cols: &[u64]) -> u64 {
  for s in 1..cols.len() {
    let (a, b) = cols.split_at(s);
    if a.into_iter().rev().zip(b).all(|(x, y)| x == y) {
      return s as u64;
    }
  }

  for s in 1..rows.len() {
    let (a, b) = rows.split_at(s);
    if a.into_iter().rev().zip(b).all(|(x, y)| x == y) {
      return s as u64 * 100;
    }
  }

  panic!("impossible")
}

fn p2(mut rows: Vec<u64>, mut cols: Vec<u64>, bad: u64) -> u64 {
  for i in 0..rows.len() {
    for j in 0..cols.len() {
      let j = cols.len() - 1 - j;
      rows[i] ^= 1 << j;
      cols[j] ^= 1 << (rows.len() - 1 - i);

      for s in 1..cols.len() {
        let (a, b) = cols.split_at(s);
        if a.into_iter().rev().zip(b).all(|(x, y)| x == y) {
          let ret = s as u64;
          if ret != bad {
            return ret;
          }
        }
      }

      for s in 1..rows.len() {
        let (a, b) = rows.split_at(s);
        if a.into_iter().rev().zip(b).all(|(x, y)| x == y) {
          let ret = s as u64 * 100;
          if ret != bad {
            return ret;
          }
        }
      }

      // revert
      rows[i] ^= 1 << j;
      cols[j] ^= 1 << (rows.len() - 1 - i);
    }
  }

  panic!("impossible")
}

fn digit(c: char) -> u64 {
  match c {
    '.' => 0,
    '#' => 1,
    _ => panic!("bad input"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    assert_eq!((405, 400), solve(input));
  }
}
