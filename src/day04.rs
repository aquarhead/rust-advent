struct Card {
  wins: Vec<u32>,
  have: Vec<u32>,
}

impl Card {
  fn matches(&self) -> usize {
    self.have.iter().filter(|n| self.wins.contains(n)).count()
  }
}

pub fn solve(input: String) -> (u32, u32) {
  let cards: Vec<Card> = input
    .trim()
    .lines()
    .map(|line| {
      let (_card, n_str) = line.split_once(": ").unwrap();
      let (wins, have) = n_str.split_once("|").unwrap();
      Card {
        wins: wins.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect(),
        have: have.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect(),
      }
    })
    .collect();

  let p1 = cards
    .iter()
    .map(|c| match c.matches() {
      0 => 0,
      n => 1 << (n - 1),
    })
    .sum();

  let mut num_cards = vec![1; cards.len()];

  for (idx, c) in cards.iter().enumerate() {
    for n in 1..=c.matches() {
      num_cards[idx + n] += num_cards[idx]
    }
  }

  (p1, num_cards.into_iter().sum())
}
