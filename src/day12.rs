use rayon::prelude::*;
use std::collections::HashMap;

pub fn solve(input: &str) -> (u64, u64) {
  let lines = input
    .trim()
    .lines()
    .map(|x| x.to_string())
    .collect::<Vec<_>>();

  let p1 = lines.clone().into_par_iter().map(|x| p1(&x)).sum();
  let p2 = lines.into_par_iter().map(|x| p2(&x)).sum();

  (p1, p2)
}

fn p1(row: &str) -> u64 {
  let (conds, record) = row.trim().split_once(' ').unwrap();
  let record: Vec<_> = record
    .split(',')
    .map(|x| x.parse::<u64>().unwrap())
    .collect();

  // add a '.' on both ends to help with matching
  let mut c = String::from(".");
  c.push_str(conds);
  c.push('.');

  d(c, &record)
}

fn p2(row: &str) -> u64 {
  let (conds, record) = row.trim().split_once(' ').unwrap();
  let record: Vec<_> = record
    .split(',')
    .map(|x| x.parse::<u64>().unwrap())
    .collect();

  let record = record.repeat(5);

  // add a '.' on both ends to help with matching
  let mut c = String::from('.');
  c.push_str(conds);
  for _ in 0..4 {
    c.push('?');
    c.push_str(conds);
  }
  c.push('.');

  d(c, &record)
}

fn d(c: String, record: &[u64]) -> u64 {
  // key: idx in c, count of completed matching in record, current continious # length
  let mut arr: HashMap<(usize, usize, u64), u64> = HashMap::new();
  arr.insert((0, 0, 0), 1);

  for (i, ch) in c.char_indices().skip(1) {
    if ch == '.' || ch == '?' {
      *arr.entry((i, 0, 0)).or_insert(0) +=
        arr.get(&(i - 1, 0, 0)).cloned().unwrap_or(0);

      for j in 1..=record.len() {
        // continue .
        let a = arr.get(&(i - 1, j, 0)).cloned().unwrap_or(0);
        // just closing a series of #
        let b = arr
          .get(&(i - 1, j - 1, record[j - 1]))
          .cloned()
          .unwrap_or(0);

        *arr.entry((i, j, 0)).or_insert(0) += a + b;
      }
    }
    if ch == '#' || ch == '?' {
      for j in 0..record.len() {
        for k in 1..=record[j] {
          // continue #
          *arr.entry((i, j, k)).or_insert(0) +=
            arr.get(&(i - 1, j, k - 1)).cloned().unwrap_or(0);
        }
      }
    }
  }

  let k = (c.len() - 1, record.len(), 0);

  *arr.get(&k).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_p1() {
    assert_eq!(1, p1("???.### 1,1,3"));
    assert_eq!(4, p1(".??..??...?##. 1,1,3"));
    assert_eq!(1, p1("?#?#?#?#?#?#?#? 1,3,1,6"));
    assert_eq!(1, p1("????.#...#... 4,1,1"));
    assert_eq!(4, p1("????.######..#####. 1,6,5"));
    assert_eq!(10, p1("?###???????? 3,2,1"));

    assert_eq!(1, p1("#..#?#?.?????#.# 1,1,2,5,1"));
  }

  #[test]
  fn test_p2() {
    assert_eq!(1, p2("???.### 1,1,3"));
    assert_eq!(16384, p2(".??..??...?##. 1,1,3"));
    assert_eq!(1, p2("?#?#?#?#?#?#?#? 1,3,1,6"));
    assert_eq!(16, p2("????.#...#... 4,1,1"));
    assert_eq!(2500, p2("????.######..#####. 1,6,5"));
    assert_eq!(506250, p2("?###???????? 3,2,1"));
  }
}
