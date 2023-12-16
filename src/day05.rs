pub fn solve(input: String) -> (u64, u64) {
  let mut parts = input.trim().split("\n\n");
  let seeds: Vec<u64> = parts
    .next()
    .unwrap()
    .split_once(": ")
    .unwrap()
    .1
    .split_ascii_whitespace()
    .map(|x| x.parse::<u64>().unwrap())
    .collect();

  let mappings = parts
    .map(|part| {
      part
        .lines()
        .skip(1)
        .map(|range_str| {
          let mut p = range_str.split_ascii_whitespace();
          (
            p.next().unwrap().parse::<u64>().unwrap(),
            p.next().unwrap().parse::<u64>().unwrap(),
            p.next().unwrap().parse::<u64>().unwrap(),
          )
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  (p1(&seeds, &mappings), p2(&seeds, &mappings))
}

fn p1(seeds: &Vec<u64>, mappings: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
  let mut src = seeds.clone();

  for ranges in mappings {
    let mut dest = vec![];
    for (dest_start, src_start, range_len) in ranges.iter().copied() {
      let (mapped, rest): (Vec<_>, Vec<_>) = src
        .into_iter()
        .partition(|s| *s >= src_start && *s < src_start + range_len);

      dest.extend(mapped.into_iter().map(|m| dest_start + (m - src_start)));
      src = rest;
    }

    src.extend(dest);
  }

  src.into_iter().min().unwrap()
}

fn p2(seeds: &Vec<u64>, mappings: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
  let mut src = {
    let mut acc = vec![];
    let mut i = seeds.iter();
    while let Some(start) = i.next() {
      let len = i.next().unwrap();
      acc.push((*start, start + len - 1));
    }
    acc
  };

  for ranges in mappings {
    let mut dest = vec![];
    for (dest_start, src_start, range_len) in ranges.iter().copied() {
      let mut rest = vec![];
      while let Some((mut begin, mut end)) = src.pop() {
        if (begin >= src_start + range_len) || (end < src_start) {
          rest.push((begin, end));
          continue;
        }

        if begin < src_start {
          rest.push((begin, src_start - 1));
          begin = src_start;
        }

        if end >= src_start + range_len {
          rest.push((src_start + range_len, end));
          end = src_start + range_len - 1;
        }

        dest.push((
          dest_start + (begin - src_start),
          dest_start + (end - src_start),
        ));
      }
      src = rest;
    }

    src.extend(dest);
  }

  src.into_iter().map(|(a, _)| a).min().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
    "#;

    assert_eq!((35, 46), solve(input.to_string()));
  }
}
