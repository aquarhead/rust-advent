use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleType {
  Broadcaster,
  FlipFlop(bool),
  Conjunction,
}

use ModuleType::*;

type Wire = (String, String, bool);

pub fn solve(input: &str) -> (u64, u64) {
  let mut modules = HashMap::new();
  let mut wires: Vec<Wire> = Vec::new();
  for line in input.trim().lines() {
    let (idtype, targets) = line.split_once(" -> ").unwrap();

    let (mid, mt) = match &idtype[0..1] {
      "b" => (idtype, Broadcaster),
      "%" => (&idtype[1..], FlipFlop(false)),
      "&" => (&idtype[1..], Conjunction),
      _ => panic!("impossible"),
    };

    modules.insert(mid.to_string(), mt);

    for g in targets.split(", ") {
      wires.push((mid.to_string(), g.to_string(), false));
    }
  }

  wires.push(("button".to_string(), "broadcaster".to_string(), false));

  (p1(modules.clone(), wires.clone()), p2(modules, wires))
}

fn p1(mut modules: HashMap<String, ModuleType>, mut wires: Vec<Wire>) -> u64 {
  let mut p1 = vec![0; 2];
  for _ in 0..1000 {
    let mut work = VecDeque::from([(wires.len() - 1, false)]);
    while let Some((cur, pulse)) = work.pop_front() {
      p1[pulse as usize] += 1;
      wires[cur].2 = pulse;
      let mid = wires[cur].1.clone();

      match modules.get_mut(&mid) {
        Some(Broadcaster) => wires
          .iter()
          .enumerate()
          .filter(|(_, w)| w.0 == mid)
          .for_each(|(i, _)| work.push_back((i, pulse))),
        Some(FlipFlop(state)) => {
          if !pulse {
            let np = !*state;
            *state = np;
            wires
              .iter()
              .enumerate()
              .filter(|(_, w)| w.0 == mid)
              .for_each(|(i, _)| work.push_back((i, np)));
          }
        }
        Some(Conjunction) => {
          let np = wires.iter().filter(|w| w.1 == mid).any(|w| !w.2);
          wires
            .iter()
            .enumerate()
            .filter(|(_, w)| w.0 == mid)
            .for_each(|(i, _)| work.push_back((i, np)));
        }
        None => {}
      }
    }
  }
  p1[0] * p1[1]
}

fn p2(mut modules: HashMap<String, ModuleType>, mut wires: Vec<Wire>) -> u64 {
  let mut rets = vec![0; 4];
  for ret in 1.. {
    let mut vg = 0;
    let mut nb = 0;
    let mut vc = 0;
    let mut ls = 0;
    let mut work = VecDeque::from([(wires.len() - 1, false)]);
    while let Some((cur, pulse)) = work.pop_front() {
      wires[cur].2 = pulse;
      let mid = wires[cur].1.clone();
      if mid == "vg" && !pulse {
        vg += 1;
      }
      if mid == "nb" && !pulse {
        nb += 1;
      }
      if mid == "vc" && !pulse {
        vc += 1;
      }
      if mid == "ls" && !pulse {
        ls += 1;
      }

      match modules.get_mut(&mid) {
        Some(Broadcaster) => wires
          .iter()
          .enumerate()
          .filter(|(_, w)| w.0 == mid)
          .for_each(|(i, _)| work.push_back((i, pulse))),
        Some(FlipFlop(state)) => {
          if !pulse {
            let np = !*state;
            *state = np;
            wires
              .iter()
              .enumerate()
              .filter(|(_, w)| w.0 == mid)
              .for_each(|(i, _)| work.push_back((i, np)));
          }
        }
        Some(Conjunction) => {
          let np = wires.iter().filter(|w| w.1 == mid).any(|w| !w.2);
          wires
            .iter()
            .enumerate()
            .filter(|(_, w)| w.0 == mid)
            .for_each(|(i, _)| work.push_back((i, np)));
        }
        None => {}
      }
    }

    if vg == 1 {
      rets[0] = ret
    }
    if nb == 1 {
      rets[1] = ret
    }
    if vc == 1 {
      rets[2] = ret
    }
    if ls == 1 {
      rets[3] = ret
    }
    if rets.iter().all(|x| *x > 0) {
      break;
    }
  }

  rets[0] * rets[1] * rets[2] * rets[3]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = r#"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
    "#;

    assert_eq!(32000000, solve(input).0);
  }

  #[test]
  fn test_example2() {
    let input = r#"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
    "#;

    assert_eq!(11687500, solve(input).0);
  }
}
