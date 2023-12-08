use std::collections::HashMap;

pub fn solve(input: &str) -> (u32, u64) {
  let (inst, map) = input.trim().split_once("\n\n").unwrap();

  let inst = inst.chars().collect::<Vec<_>>();
  let map = map.trim().lines().fold(HashMap::new(), |mut acc, line| {
    let (from, to) = line.split_once(" = ").unwrap();
    let to = to
      .trim_start_matches("(")
      .trim_end_matches(")")
      .split_once(", ")
      .unwrap();
    acc.insert(from, to);

    acc
  });

  let mut loc = "AAA";
  let mut p1 = 0;
  while loc != "ZZZ" {
    match inst[p1 % inst.len()] {
      'L' => loc = map.get(loc).unwrap().0,
      'R' => loc = map.get(loc).unwrap().1,
      _ => panic!("impossible"),
    }
    p1 += 1;
  }

  let locs = map.keys().filter(|k| k.ends_with('A')).map(|x| *x).collect::<Vec<_>>();
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

  let mut p2 = steps[0] as u64;
  while steps.iter().any(|s| p2 % *s != 0) {
    p2 += steps[0];
  }

  (p1 as u32, p2)
}
