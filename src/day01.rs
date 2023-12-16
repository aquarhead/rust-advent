pub fn solve() {
  let content =
    std::fs::read_to_string("inputs/day01.txt").expect("read input");

  let p1: u32 = content
    .trim()
    .lines()
    .map(|line| {
      let mut nums = line.matches(char::is_numeric);
      let first = nums
        .next()
        .expect("should have first number")
        .parse::<u32>()
        .expect("parse first number");
      let mut last = first;
      for n in nums {
        last = n.parse::<u32>().expect("parse number");
      }
      first * 10 + last
    })
    .sum();

  println!("part1: {}", p1);

  let str_digit = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
  ];

  let p2: u32 = content
    .trim()
    .lines()
    .map(|line| {
      let mut nums = line
        .match_indices(char::is_numeric)
        .map(|(idx, num)| (idx, num.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();
      for (n, s) in str_digit.into_iter().enumerate() {
        nums.extend(line.match_indices(s).map(|(idx, _)| (idx, n as u32 + 1)));
      }

      nums.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

      nums.first().unwrap().1 * 10 + nums.last().unwrap().1
    })
    .sum();

  println!("part2: {}", p2);
}
