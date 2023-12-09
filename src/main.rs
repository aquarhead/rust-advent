#![allow(dead_code)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() {
  let content = std::fs::read_to_string("inputs/day09.txt").expect("read file");
  let (p1, p2) = day09::solve(&content);

  println!("part1: {}", p1);
  println!("part2: {}", p2);
}
