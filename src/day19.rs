use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

type Part = (u64, u64, u64, u64);

pub fn solve(input: &str) -> (u64, u64) {
  let (wf_str, parts_str) = input.trim().split_once("\n\n").unwrap();
  let workflows: HashMap<_, _> = wf_str
    .lines()
    .map(|line| {
      let (workflow, rules_default) = line.split_once('{').unwrap();
      let (rules, default_target) = rules_default.rsplit_once(',').unwrap();
      let default_target = default_target.strip_suffix('}').unwrap();
      let rules = rules
        .split(',')
        .map(|rs| {
          let (cond, target) = rs.split_once(':').unwrap();
          let k = cond.chars().next().unwrap();
          let sym = &cond[1..2];

          let er = match sym {
            "<" => Ordering::Less,
            ">" => Ordering::Greater,
            _ => panic!("impossible",),
          };

          let v = cond[2..].parse::<u64>().unwrap();

          (k, er, v, target.to_string())
        })
        .collect::<Vec<_>>();

      (workflow.to_string(), (rules, default_target.to_string()))
    })
    .collect();

  let parts = parts_str
    .lines()
    .map(|line| {
      let mut params = line.strip_suffix('}').unwrap().split(',');
      let x = {
        let t = params.next().unwrap();
        let (_, v) = t.split_once('=').unwrap();
        v.parse::<u64>().unwrap()
      };
      let m = {
        let t = params.next().unwrap();
        let (_, v) = t.split_once('=').unwrap();
        v.parse::<u64>().unwrap()
      };
      let a = {
        let t = params.next().unwrap();
        let (_, v) = t.split_once('=').unwrap();
        v.parse::<u64>().unwrap()
      };
      let s = {
        let t = params.next().unwrap();
        let (_, v) = t.split_once('=').unwrap();
        v.parse::<u64>().unwrap()
      };
      (x, m, a, s)
    })
    .collect::<Vec<_>>();

  let p1 = parts
    .into_iter()
    .filter_map(|p| {
      let mut wf = "in";
      'outer: while wf != "A" && wf != "R" {
        let (rules, dt) = workflows.get(wf).unwrap();
        for (k, er, v, t) in rules {
          let a = match k {
            'x' => p.0,
            'm' => p.1,
            'a' => p.2,
            's' => p.3,
            _ => panic!("impossible"),
          };
          if a.cmp(&v) == *er {
            wf = t;
            continue 'outer;
          }
        }
        wf = dt;
      }

      if wf == "A" {
        Some(p.0 + p.1 + p.2 + p.3)
      } else {
        None
      }
    })
    .sum();

  // p2
  let mut p2 = 0;
  let mut search = {
    let all: HashSet<u16> = HashSet::from_iter(1..=4000);
    let init = HashMap::from([
      ('x', all.clone()),
      ('m', all.clone()),
      ('a', all.clone()),
      ('s', all),
    ]);
    vec![("in".to_string(), init)]
  };

  while let Some((wfid, range)) = search.pop() {
    match wfid.as_str() {
      "A" => {
        p2 += range.values().map(|s| s.len()).product::<usize>() as u64;
        continue;
      }
      "R" => continue,
      _ => {}
    }

    let (rules, default) = workflows.get(&wfid).unwrap();

    let default_range = rules.iter().fold(range, |mut acc, rule| {
      let mut rest = acc.clone();
      // match
      (*acc.get_mut(&rule.0).unwrap())
        .retain(|a| a.cmp(&(rule.2 as u16)) == rule.1);

      search.push((rule.3.to_string(), acc));

      // no match
      (*rest.get_mut(&rule.0).unwrap())
        .retain(|a| a.cmp(&(rule.2 as u16)) != rule.1);

      rest
    });

    // default
    search.push((default.clone(), default_range));
  }

  (p1, p2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = r#"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
    "#;

    assert_eq!((19114, 167409079868000), solve(input));
  }
}
