use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

type Pos = (i32, i32);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn left(&self) -> Self {
    match self {
      Up => Left,
      Left => Down,
      Down => Right,
      Right => Up,
    }
  }

  fn right(&self) -> Self {
    match self {
      Up => Right,
      Right => Down,
      Down => Left,
      Left => Up,
    }
  }

  fn mov(&self, p: Pos) -> Pos {
    match self {
      Up => (p.0 - 1, p.1),
      Down => (p.0 + 1, p.1),
      Left => (p.0, p.1 - 1),
      Right => (p.0, p.1 + 1),
    }
  }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct StatefulPos {
  p: Pos,
  dir: Direction,
  consec: u8,
}

use Direction::*;

pub fn solve(input: &str) -> (u32, u32) {
  let mut map = HashMap::new();
  let mut k = (0, 0);
  for (r, line) in input.trim().lines().enumerate() {
    for (c, v) in line.char_indices() {
      k = (r as i32, c as i32);
      map.insert(k, v.to_digit(10).unwrap());
    }
  }

  let in_bound =
    |p: Pos| -> bool { p.0 >= 0 && p.0 <= k.0 && p.1 >= 0 && p.1 <= k.1 };

  let p1 = dijkstra(
    &StatefulPos {
      p: (0, 0),
      dir: Right,
      consec: 0,
    },
    |sp| {
      let mut next = vec![];
      if sp.consec < 3 {
        let np = sp.dir.mov(sp.p);
        if in_bound(np) {
          next.push((
            StatefulPos {
              p: np,
              dir: sp.dir,
              consec: sp.consec + 1,
            },
            *map.get(&np).unwrap(),
          ));
        }
      }
      {
        let nd = sp.dir.left();
        let np = nd.mov(sp.p);
        if in_bound(np) {
          next.push((
            StatefulPos {
              p: np,
              dir: nd,
              consec: 1,
            },
            *map.get(&np).unwrap(),
          ));
        }
      }
      {
        let nd = sp.dir.right();
        let np = nd.mov(sp.p);
        if in_bound(np) {
          next.push((
            StatefulPos {
              p: np,
              dir: nd,
              consec: 1,
            },
            *map.get(&np).unwrap(),
          ));
        }
      }

      next
    },
    |sp| sp.p == k,
  )
  .unwrap()
  .1;

  let p2 = dijkstra(
    &StatefulPos {
      p: (0, 0),
      dir: Right,
      consec: 0,
    },
    |sp| {
      let mut next = vec![];
      if sp.consec < 10 {
        let np = sp.dir.mov(sp.p);
        if in_bound(np) {
          next.push((
            StatefulPos {
              p: np,
              dir: sp.dir,
              consec: sp.consec + 1,
            },
            *map.get(&np).unwrap(),
          ));
        }
      }
      if sp.consec >= 4 {
        {
          let nd = sp.dir.left();
          let np = nd.mov(sp.p);
          if in_bound(np) {
            next.push((
              StatefulPos {
                p: np,
                dir: nd,
                consec: 1,
              },
              *map.get(&np).unwrap(),
            ));
          }
        }
        {
          let nd = sp.dir.right();
          let np = nd.mov(sp.p);
          if in_bound(np) {
            next.push((
              StatefulPos {
                p: np,
                dir: nd,
                consec: 1,
              },
              *map.get(&np).unwrap(),
            ));
          }
        }
      }

      next
    },
    |sp| sp.p == k,
  )
  .unwrap()
  .1;

  (p1, p2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_exmple() {
    let input = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    assert_eq!((102, 94), solve(input));
  }
}
