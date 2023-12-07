use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOAK,
  FullHouse,
  FourOAK,
  FiveOAK,
}

impl HandType {
  fn from_hand(hand: &str) -> Self {
    let grp = hand.chars().fold(HashMap::<char, u8>::new(), |mut acc, ch| {
      acc.entry(ch).and_modify(|x| *x += 1).or_insert(1);
      acc
    });

    match grp.len() {
      1 => Self::FiveOAK,
      2 => match *grp.iter().next().unwrap().1 {
        1 | 4 => Self::FourOAK,
        2 | 3 => Self::FullHouse,
        _ => panic!("no way"),
      },
      3 => {
        if grp.iter().any(|(_, x)| *x == 3) {
          Self::ThreeOAK
        } else {
          Self::TwoPair
        }
      }
      4 => Self::OnePair,
      5 => Self::HighCard,
      _ => panic!("impossible"),
    }
  }
}

#[derive(Debug, Clone)]
struct Hand {
  typ: HandType,
  cards: (char, char, char, char, char),
  bid: u64,
}

impl Hand {
  fn parse(hand_bid: &str) -> Self {
    let (hand, bid) = hand_bid.split_once(" ").unwrap();
    let typ = HandType::from_hand(hand);
    let mut hand = hand.chars();
    let cards = (
      hand.next().unwrap(),
      hand.next().unwrap(),
      hand.next().unwrap(),
      hand.next().unwrap(),
      hand.next().unwrap(),
    );
    let bid = bid.parse().unwrap();

    Self { typ, cards, bid }
  }
}

pub fn solve(input: &str) -> u64 {
  let hands: Vec<_> = input.trim().lines().map(Hand::parse).collect();

  p1(hands.clone())
}

fn p1(mut hands: Vec<Hand>) -> u64 {
  hands.sort_by(|a, b| {
    [
      a.typ.cmp(&b.typ),
      char_ord(a.cards.0).cmp(&char_ord(b.cards.0)),
      char_ord(a.cards.1).cmp(&char_ord(b.cards.1)),
      char_ord(a.cards.2).cmp(&char_ord(b.cards.2)),
      char_ord(a.cards.3).cmp(&char_ord(b.cards.3)),
      char_ord(a.cards.4).cmp(&char_ord(b.cards.4)),
    ]
    .into_iter()
    .skip_while(|x| x == &Ordering::Equal)
    .next()
    .unwrap()
  });

  hands
    .into_iter()
    .enumerate()
    .fold(0, |acc, (idx, hand)| acc + (idx as u64 + 1) * hand.bid)
}

fn char_ord(a: char) -> u8 {
  match a {
    '2' => 0,
    '3' => 1,
    '4' => 2,
    '5' => 3,
    '6' => 4,
    '7' => 5,
    '8' => 6,
    '9' => 7,
    'T' => 8,
    'J' => 9,
    'Q' => 10,
    'K' => 11,
    'A' => 12,
    _ => panic!("impossible"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
    "#;

    assert_eq!(6440, solve(input));
  }
}
