#![allow(dead_code)]

mod day01;
mod day02;
mod day03;

fn main() {
  let content = std::fs::read_to_string("inputs/day03.txt").expect("read file");
  let (p1, p2) = day03::solve(content);

  println!("part1: {}", p1);
  println!("part2: {}", p2);
}
