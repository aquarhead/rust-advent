#![allow(dead_code)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
  let content = std::fs::read_to_string("inputs/day06.txt").expect("read file");
  let (p1, p2) = day06::solve(&content);

  println!("part1: {}", p1);
  println!("part2: {}", p2);
}
