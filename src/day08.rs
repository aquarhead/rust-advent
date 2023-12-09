use std::collections::HashMap;

pub fn solve(input: &str) -> (u64, u64) {
  let (inst, map) = input.trim().split_once("\n\n").unwrap();

  let inst = inst.chars().collect::<Vec<_>>();
  let map = map
    .trim()
    .lines()
    .map(|line| {
      (
        line.get(0..=2).unwrap(),
        (line.get(7..=9).unwrap(), line.get(12..=14).unwrap()),
      )
    })
    .collect::<HashMap<_, _>>();

  let mut locs = map.keys().filter(|k| k.ends_with('A')).map(|x| *x).collect::<Vec<_>>();
  locs.sort();
  let steps = locs
    .into_iter()
    .map(|mut loc| {
      let mut steps = 0;
      while !loc.ends_with('Z') {
        match inst[steps % inst.len()] {
          'L' => loc = map.get(loc).unwrap().0,
          'R' => loc = map.get(loc).unwrap().1,
          _ => panic!("impossible"),
        }
        steps += 1;
      }
      steps as u64
    })
    .collect::<Vec<_>>();

  let mut p2 = steps[0];
  while steps.iter().any(|s| p2 % *s != 0) {
    p2 += steps[0];
  }

  (steps[0], p2)
}
