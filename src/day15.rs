pub fn solve(input: &str) -> (u64, u64) {
  let p1 = input.trim().split(',').map(hash).sum();

  let p2 = input
    .trim()
    .split(',')
    .fold(vec![Vec::new(); 256], |mut boxes, inst| {
      let (label, opt_val) = inst.split_once(['=', '-']).unwrap();
      let b = hash(label) as usize;
      if let Ok(v) = opt_val.parse::<u64>() {
        if let Some(found) = boxes[b].iter().position(|(k, _)| *k == label) {
          boxes[b][found] = (label, v);
        } else {
          boxes[b].push((label, v));
        }
      } else {
        if let Some(found) = boxes[b].iter().position(|(k, _)| *k == label) {
          boxes[b].remove(found);
        }
      }

      boxes
    })
    .into_iter()
    .enumerate()
    .map(|(bi, b)| -> u64 {
      let s: u64 = b
        .into_iter()
        .enumerate()
        .map(|(i, (_, f))| (i + 1) as u64 * f)
        .sum();

      s * (bi + 1) as u64
    })
    .sum();

  (p1, p2)
}

fn hash(s: &str) -> u64 {
  s.chars().fold(0, |mut cur, c| {
    cur += (c as u8) as u64;
    cur *= 17;
    cur % 256
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    assert_eq!((1320, 145), solve(input));
  }
}
