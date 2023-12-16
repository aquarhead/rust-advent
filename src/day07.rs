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
    let grp = hand
      .chars()
      .fold(HashMap::<char, u8>::new(), |mut acc, ch| {
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

  fn from_hand_joker(hand: &str) -> Self {
    let (grp, jokers) =
      hand
        .chars()
        .fold((HashMap::<char, u8>::new(), 0), |mut acc, ch| {
          if ch == 'J' {
            acc.1 += 1;
          } else {
            acc.0.entry(ch).and_modify(|x| *x += 1).or_insert(1);
          }

          acc
        });

    match grp.len() {
      // all joker
      0 => Self::FiveOAK,
      // all joker can convert to the same card
      1 => Self::FiveOAK,
      // 2 distinct, non-joker cards
      2 => {
        match *grp.values().max().unwrap() {
          // xyJJJ -> xyyyy
          1 => Self::FourOAK,
          2 => match jokers {
            // xxyyJ -> xxyyy
            1 => Self::FullHouse,
            // xyyJJ -> xyyyy
            2 => Self::FourOAK,
            _ => panic!("impossible"),
          },
          3 => {
            match jokers {
              // xxyyy
              0 => Self::FullHouse,
              // xyyyJ
              1 => Self::FourOAK,
              _ => panic!("impossible"),
            }
          }
          4 => Self::FourOAK,
          _ => panic!("impossible"),
        }
      }
      // 3 distinct, non-joker cards
      3 => {
        match *grp.values().max().unwrap() {
          // xyzJJ -> xyzzz
          1 => Self::ThreeOAK,
          2 => match jokers {
            // xyyzz
            0 => Self::TwoPair,
            // xyzzJ -> xyzzz
            1 => Self::ThreeOAK,
            _ => panic!("impossible"),
          },
          // xyzzz
          3 => Self::ThreeOAK,
          _ => panic!("impossible"),
        }
      }
      // always one pair ??
      4 => Self::OnePair,
      // no joker
      5 => Self::HighCard,
      _ => panic!("impossible"),
    }
  }
}

#[derive(Debug, Clone)]
struct Hand {
  typ: HandType,
  typ_joker: HandType,
  cards: (char, char, char, char, char),
  bid: u64,
}

impl Hand {
  fn parse(hand_bid: &str) -> Self {
    let (hand, bid) = hand_bid.split_once(' ').unwrap();
    let typ = HandType::from_hand(hand);
    let typ_joker = HandType::from_hand_joker(hand);
    let mut hand = hand.chars();
    let cards = (
      hand.next().unwrap(),
      hand.next().unwrap(),
      hand.next().unwrap(),
      hand.next().unwrap(),
      hand.next().unwrap(),
    );
    let bid = bid.parse().unwrap();

    Self {
      typ,
      typ_joker,
      cards,
      bid,
    }
  }
}

pub fn solve(input: &str) -> (u64, u64) {
  let hands: Vec<_> = input.trim().lines().map(Hand::parse).collect();

  (p1(hands.clone()), p2(hands.clone()))
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
    .find(|x| x != &Ordering::Equal)
    .unwrap()
  });

  hands
    .into_iter()
    .enumerate()
    .fold(0, |acc, (idx, hand)| acc + (idx as u64 + 1) * hand.bid)
}

fn p2(mut hands: Vec<Hand>) -> u64 {
  hands.sort_by(|a, b| {
    [
      a.typ_joker.cmp(&b.typ_joker),
      char_ord_joker(a.cards.0).cmp(&char_ord_joker(b.cards.0)),
      char_ord_joker(a.cards.1).cmp(&char_ord_joker(b.cards.1)),
      char_ord_joker(a.cards.2).cmp(&char_ord_joker(b.cards.2)),
      char_ord_joker(a.cards.3).cmp(&char_ord_joker(b.cards.3)),
      char_ord_joker(a.cards.4).cmp(&char_ord_joker(b.cards.4)),
    ]
    .into_iter()
    .find(|x| x != &Ordering::Equal)
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

fn char_ord_joker(a: char) -> u8 {
  match a {
    'J' => 0,
    '2' => 1,
    '3' => 2,
    '4' => 3,
    '5' => 4,
    '6' => 5,
    '7' => 6,
    '8' => 7,
    '9' => 8,
    'T' => 9,
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

    assert_eq!((6440, 5905), solve(input));
  }
}
