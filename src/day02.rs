use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{digit1, space1},
  combinator::map_res,
  multi::separated_list1,
  IResult,
};

pub fn solve() {
  let content = std::fs::read_to_string("inputs/day02.txt").expect("read input");

  let mut p1: u32 = 0;
  let mut p2: u32 = 0;

  for line in content.trim().lines() {
    let (_, g) = game(line).expect("parse game");

    if g.1.iter().copied().all(|(r, g, b)| r <= 12 && g <= 13 && b <= 14) {
      p1 += g.0;
    }

    let min_cubes = g
      .1
      .iter()
      .copied()
      .fold((0, 0, 0), |acc, s| (acc.0.max(s.0), acc.1.max(s.1), acc.2.max(s.2)));

    p2 += min_cubes.0 * min_cubes.1 * min_cubes.2;
  }

  println!("part1: {}", p1);
  println!("part2: {}", p2);
}

type Set = (u32, u32, u32);
type Game = (u32, Vec<Set>);

fn cubes(input: &str) -> IResult<&str, (u32, &str)> {
  let (input, n) = map_res(digit1, str::parse)(input)?;
  let (input, _) = space1(input)?;
  let (input, color) = alt((tag("red"), tag("green"), tag("blue")))(input)?;

  Ok((input, (n, color)))
}

fn set(input: &str) -> IResult<&str, Set> {
  let (input, cbs) = separated_list1(tag(", "), cubes)(input)?;
  let set = cbs.into_iter().fold((0, 0, 0), |mut acc, item| {
    match item.1 {
      "red" => acc.0 += item.0,
      "green" => acc.1 += item.0,
      "blue" => acc.2 += item.0,
      _ => panic!("bad color"),
    }
    acc
  });

  Ok((input, set))
}

fn game(input: &str) -> IResult<&str, Game> {
  let (input, _) = tag("Game ")(input)?;
  let (input, id) = map_res(digit1, str::parse)(input)?;
  let (input, _) = tag(": ")(input)?;
  let (input, sets) = separated_list1(tag("; "), set)(input)?;

  Ok((input, (id, sets)))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_game() {
    assert_eq!(
      Ok(("", (100, vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]))),
      game("Game 100: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
    );
  }

  #[test]
  fn test_set() {
    assert_eq!(Ok(("", (1, 2, 6))), set("1 red, 2 green, 6 blue"))
  }

  #[test]
  fn test_cubes() {
    assert_eq!(Ok(("", (3, "blue"))), cubes("3 blue"));
  }
}
