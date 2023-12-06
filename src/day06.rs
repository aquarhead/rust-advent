pub fn solve(input: &str) -> (u64, u64) {
  let (times_str, records_str) = input.trim().split_once("\n").unwrap();
  let times_str = times_str.split_once(": ").unwrap().1.trim();
  let records_str = records_str.split_once(": ").unwrap().1.trim();

  let times = times_str.split_ascii_whitespace().map(|n| n.parse::<u64>().unwrap());
  let records = records_str.split_ascii_whitespace().map(|n| n.parse::<u64>().unwrap());

  let p1 = times.zip(records).map(|(s, r)| p2(s, r)).fold(1, |acc, n| acc * n);

  let secs = times_str.replace(" ", "").parse::<u64>().unwrap();
  let r = records_str.replace(" ", "").parse::<u64>().unwrap();
  (p1, p2(secs, r))
}

fn p2(secs: u64, record: u64) -> u64 {
  let n = (1..).take_while(|hold| (secs - hold) * hold <= record).count() as u64;
  secs - n - n - 1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_races() {
    assert_eq!(4, p2(7, 9));
    assert_eq!(8, p2(15, 40));
    assert_eq!(9, p2(30, 200));
  }

  #[test]
  fn test_p2() {
    assert_eq!(71503, p2(71530, 940200));
  }
}
