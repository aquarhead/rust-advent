#![allow(dead_code)]

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
  let content = std::fs::read_to_string("inputs/day04.txt").expect("read file");
  let (p1, p2) = day04::solve(content);

  println!("part1: {}", p1);
  println!("part2: {}", p2);
}
