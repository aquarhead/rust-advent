pub fn solve(input: &str) -> (u64, u64) {
  let p1 = input
    .trim()
    .split("\n\n")
    .map(|pattern| {
      //
      p1(pattern)
    })
    .sum();

  (p1, 0)
}

fn p1(pattern: &str) -> u64 {
  let mut rows = Vec::new();
  let mut cols = Vec::new();
  for line in pattern.trim().lines() {
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
    assert_eq!((405, 0), solve(input));
  }
}
