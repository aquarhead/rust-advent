use rayon::prelude::*;

pub fn solve(input: &str) -> (i64, i64) {
  let hists: Vec<_> = input
    .trim()
    .lines()
    .map(|line| {
      line
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
    })
    .collect();

  let p1 = hists.clone().into_par_iter().map(p1).sum();
  let p2 = hists.into_par_iter().map(p2).sum();

  (p1, p2)
}

fn p1(mut diffs: Vec<i64>) -> i64 {
  let mut res = 0;
  while diffs.iter().any(|x| *x != 0) {
    for i in 0..(diffs.len() - 1) {
      diffs[i] = diffs[i + 1] - diffs[i];
    }
    res += diffs.pop().unwrap();
  }

  res
}

fn p2(mut diffs: Vec<i64>) -> i64 {
  let mut heads = Vec::new();
  while diffs.iter().any(|x| *x != 0) {
    heads.push(diffs[0]);
    for i in 0..(diffs.len() - 1) {
      diffs[i] = diffs[i + 1] - diffs[i];
    }
    let _ = diffs.pop();
  }

  heads.into_iter().rfold(0, |acc, x| x - acc)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    assert_eq!(18, p1(vec![0, 3, 6, 9, 12, 15]));
    assert_eq!(28, p1(vec![1, 3, 6, 10, 15, 21]));
    assert_eq!(68, p1(vec![10, 13, 16, 21, 30, 45]));

    assert_eq!(5, p2(vec![10, 13, 16, 21, 30, 45]));
  }
}
