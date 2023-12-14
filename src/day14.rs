use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) -> (u32, u32) {
  (p1(input), p2(input))
}

fn p1(input: &str) -> u32 {
  let mut ln = 0;
  let mut next_pos = Vec::new();
  let mut final_pos = Vec::new();
  for line in input.trim().lines() {
    ln += 1;
    // first line
    if next_pos.len() == 0 {
      next_pos = vec![0; line.len()];
    }
    for (i, ch) in line.char_indices() {
      match ch {
        'O' => {
          final_pos.push(next_pos[i]);
          next_pos[i] += 1;
        }
        '#' => next_pos[i] = ln,
        _ => {}
      }
    }
  }
  final_pos.into_iter().map(|x| ln - x).sum()
}

fn p2(input: &str) -> u32 {
  let mut movable = Vec::new();
  let mut fixed_rows = Vec::new();
  let mut fixed_cols = Vec::new();
  for (r, line) in input.trim().lines().enumerate() {
    fixed_rows.push(VecDeque::new());
    if fixed_cols.len() == 0 {
      fixed_cols = vec![VecDeque::new(); line.len()];
    }
    for (c, ch) in line.char_indices() {
      match ch {
        'O' => movable.push((r, c)),
        '#' => {
          fixed_rows[r].push_back(c);
          fixed_cols[c].push_back(r);
        }
        _ => {}
      }
    }
  }

  let rows = input.trim().lines().count();
  let cols = input.trim().lines().next().unwrap().len();

  let mut cycles: Vec<HashSet<(usize, usize)>> =
    vec![HashSet::from_iter(movable.clone().into_iter())];

  for _ in 0..1000000000 {
    // north
    let mut after_tilt = Vec::new();
    for c in 0..cols {
      let (incol, rest): (Vec<_>, Vec<_>) =
        movable.iter().partition(|(_, y)| *y == c);
      movable = rest;

      let mut incol = incol.into_iter().map(|(x, _)| x).collect::<Vec<_>>();
      incol.sort();

      let mut f = fixed_cols[c].clone();
      let mut tr = 0;

      for r in incol.into_iter() {
        while let Some(ff) = f.front().cloned() {
          if r > ff {
            f.pop_front();
            tr = ff + 1;
          } else {
            break;
          }
        }

        after_tilt.push((tr, c));
        tr += 1;
      }
    }
    movable = after_tilt;

    // west
    let mut after_tilt = Vec::new();
    for r in 0..rows {
      let (inrow, rest): (Vec<_>, Vec<_>) =
        movable.iter().partition(|(x, _)| *x == r);
      movable = rest;

      let mut inrow = inrow.into_iter().map(|(_, y)| y).collect::<Vec<_>>();
      inrow.sort();

      let mut f = fixed_rows[r].clone();
      let mut tc = 0;

      for c in inrow.into_iter() {
        while let Some(ff) = f.front().cloned() {
          if c > ff {
            f.pop_front();
            tc = ff + 1;
          } else {
            break;
          }
        }

        after_tilt.push((r, tc));
        tc += 1;
      }
    }
    movable = after_tilt;

    // south
    let mut after_tilt = Vec::new();
    for c in 0..cols {
      let (incol, rest): (Vec<_>, Vec<_>) =
        movable.iter().partition(|(_, y)| *y == c);
      movable = rest;

      let mut incol = incol.into_iter().map(|(x, _)| x).collect::<Vec<_>>();
      incol.sort();

      let mut f = fixed_cols[c].clone();
      let mut tr = rows - 1;

      for r in incol.into_iter().rev() {
        while let Some(ff) = f.back().cloned() {
          if r < ff {
            f.pop_back();
            tr = ff - 1;
          } else {
            break;
          }
        }

        after_tilt.push((tr, c));
        tr -= 1;
      }
    }
    movable = after_tilt;

    // east
    let mut after_tilt = Vec::new();
    for r in 0..rows {
      let (inrow, rest): (Vec<_>, Vec<_>) =
        movable.iter().partition(|(x, _)| *x == r);
      movable = rest;

      let mut inrow = inrow.into_iter().map(|(_, y)| y).collect::<Vec<_>>();
      inrow.sort();

      let mut f = fixed_rows[r].clone();
      let mut tc = cols - 1;

      for c in inrow.into_iter().rev() {
        while let Some(ff) = f.back().cloned() {
          if c < ff {
            f.pop_back();
            tc = ff - 1;
          } else {
            break;
          }
        }

        after_tilt.push((r, tc));
        tc -= 1;
      }
    }
    movable = after_tilt;

    let c: HashSet<(usize, usize)> =
      HashSet::from_iter(movable.clone().into_iter());

    if let Some(idx) = cycles.iter().position(|x| *x == c) {
      return cycles[idx + ((1000000000 - idx) % (cycles.len() - idx))]
        .iter()
        .map(|(x, _)| (rows - x) as u32)
        .sum();
    } else {
      cycles.push(c);
    }
  }

  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    assert_eq!((136, 64), solve(input));
  }
}
