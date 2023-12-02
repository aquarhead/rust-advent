pub fn solve() {
  let content = std::fs::read_to_string("inputs/day02.txt").expect("read input");

  let mut p1: u32 = 0;
  let mut p2: u32 = 0;

  for line in content.trim().lines() {
    let (game, sets_str) = line.trim().split_once(": ").expect("parse game");

    let game_id = game
      .split(' ')
      .nth(1)
      .expect("game has id")
      .parse::<u32>()
      .expect("game id is a number");

    let sets = sets_str
      .split("; ")
      .map(|set| {
        set.split(", ").fold((0, 0, 0), |mut acc, cubes| {
          match cubes.split_once(' ').expect("cubes") {
            (n, "red") => acc.0 += n.parse::<u32>().unwrap(),
            (n, "green") => acc.1 += n.parse::<u32>().unwrap(),
            (n, "blue") => acc.2 += n.parse::<u32>().unwrap(),
            _ => panic!("wat"),
          }
          acc
        })
      })
      .collect::<Vec<_>>();

    if sets.iter().copied().all(|(r, g, b)| r <= 12 && g <= 13 && b <= 14) {
      p1 += game_id;
    }

    let min_cubes = sets
      .iter()
      .copied()
      .fold((0, 0, 0), |acc, s| (acc.0.max(s.0), acc.1.max(s.1), acc.2.max(s.2)));

    p2 += min_cubes.0 * min_cubes.1 * min_cubes.2;
  }

  println!("part1: {}", p1);
  println!("part2: {}", p2);
}
