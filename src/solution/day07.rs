use std::{cmp::Ordering, collections::BTreeMap, hint::unreachable_unchecked};

use nom::{
    bytes::complete::tag, character::complete::alphanumeric1, sequence::separated_pair, IResult,
};

use crate::parse::take_number;

use super::Solution;

const CARD_ORDER: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const CARD_ORDER_WITH_JOKER: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

pub struct Day7;

impl Solution for Day7 {
    fn solve_part_one(&self, input: &super::Input) {
        let mut hands = input
            .lines()
            .filter_map(|line| {
                if !line.is_empty() {
                    Hand::try_parse(line).ok().map(|(_, hand)| hand)
                } else {
                    None
                }
            })
            .map(|hand| {
                let cards = hand.cards.iter().fold(BTreeMap::new(), |mut map, &card| {
                    let card_entry = map.entry(card).or_insert(0);
                    *card_entry += 1;
                    map
                });
                (hand, unsafe { Rules::matching(&cards) })
            })
            .collect::<Vec<_>>();

        hands.sort_by(|(a_hand, a_rule), (b_hand, b_rule)| {
            match a_rule.strength().cmp(&b_rule.strength()) {
                Ordering::Equal => a_hand
                    .cards
                    .iter()
                    .zip(&b_hand.cards)
                    .find_map(|(&a, &b)| {
                        if a == b {
                            return None;
                        }

                        let a_index = CARD_ORDER.iter().position(|c| *c == a).unwrap();
                        let b_index = CARD_ORDER.iter().position(|c| *c == b).unwrap();

                        Some(b_index.cmp(&a_index))
                    })
                    .unwrap(),
                result => result,
            }
        });

        // rules.into_iter().fold

        let solution: usize = hands
            .into_iter()
            .enumerate()
            // .inspect(|(i, (hand, _))| println!("{} * {}", hand.bid, i + 1))
            .map(|(i, (hand, _))| hand.bid * (i + 1))
            .sum();

        println!("{solution:?}");
    }

    fn solve_part_two(&self, input: &super::Input) {
        let mut hands = input
            .lines()
            .filter_map(|line| {
                if !line.is_empty() {
                    Hand::try_parse(line).ok().map(|(_, hand)| hand)
                } else {
                    None
                }
            })
            .map(|hand| {
                let cards = hand.cards.iter().fold(BTreeMap::new(), |mut map, &card| {
                    let card_entry = map.entry(card).or_insert(0);
                    *card_entry += 1;
                    map
                });

                (hand, unsafe { Rules::matching_jokers(cards) })
            })
            .collect::<Vec<_>>();

        hands.sort_by(|(a_hand, a_rule), (b_hand, b_rule)| {
            match a_rule.strength().cmp(&b_rule.strength()) {
                Ordering::Equal => a_hand
                    .cards
                    .iter()
                    .zip(&b_hand.cards)
                    .find_map(|(&a, &b)| {
                        if a == b {
                            return None;
                        }

                        let a_index = CARD_ORDER_WITH_JOKER.iter().position(|c| *c == a).unwrap();
                        let b_index = CARD_ORDER_WITH_JOKER.iter().position(|c| *c == b).unwrap();

                        Some(b_index.cmp(&a_index))
                    })
                    .unwrap(),
                result => result,
            }
        });

        // rules.into_iter().fold

        let solution: usize = hands
            .into_iter()
            .enumerate()
            // .inspect(|(i, (hand, _))| println!("{:03} *{} ({:?})", hand.bid, i + 1, hand.cards))
            .map(|(i, (hand, _))| hand.bid * (i + 1))
            .sum();

        println!("{solution:?}");
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
}

impl Hand {
    fn try_parse(input: &str) -> IResult<&str, Self> {
        let (input, (cards, bid)) = separated_pair(alphanumeric1, tag(" "), take_number)(input)?;
        let cards: Vec<char> = cards.chars().collect();
        Ok((input, Self { cards, bid }))
    }
}

#[derive(Debug)]
enum Rules {
    FiveOfKind(char),
    FourOfKind(char),
    FullHouse(char, char),
    ThreeOfKind(char),
    TwoPair(char, char),
    OnePair(char),
    HighCard,
}

impl Rules {
    /// # Safety: the caller must garantee the lenght of cards is < 5
    unsafe fn matching(cards: &BTreeMap<char, usize>) -> Self {
        if cards.len() == 1 {
            return Self::FiveOfKind(*cards.keys().next().unwrap());
        }

        let sorted = {
            let mut cards = cards.clone().into_iter().collect::<Vec<_>>();
            cards.sort_by_key(|(_, n)| -(*n as isize));
            cards
        };

        match sorted.len() {
            2 if sorted[0].1 == 4 => Self::FourOfKind(sorted[0].0),
            2 => Self::FullHouse(sorted[0].0, sorted[1].0),
            3 if sorted[0].1 == 3 => Self::ThreeOfKind(sorted[0].0),
            3 => Self::TwoPair(sorted[0].0, sorted[1].0),
            4 if sorted[0].1 == 2 => Self::OnePair(sorted[0].0),
            5 => Self::HighCard,
            _ => unreachable_unchecked(),
        }
    }

    unsafe fn matching_jokers(mut cards: BTreeMap<char, usize>) -> Self {
        if let Some(&jokers) = cards.get(&'J') {
            if let Some((_, count)) = cards
                .iter_mut()
                .filter(|(card, _)| **card != 'J')
                .max_by_key(|(_, count)| **count)
            {
                *count += jokers;
                cards.remove(&'J');
            }
        }

        if cards.len() == 1 {
            return Self::FiveOfKind(*cards.keys().next().unwrap());
        }

        let sorted = {
            let mut cards = cards.clone().into_iter().collect::<Vec<_>>();
            cards.sort_by_key(|(_, n)| -(*n as isize));
            cards
        };

        match sorted.len() {
            2 if sorted[0].1 == 4 => Self::FourOfKind(sorted[0].0),
            2 => Self::FullHouse(sorted[0].0, sorted[1].0),
            3 if sorted[0].1 == 3 => Self::ThreeOfKind(sorted[0].0),
            3 => Self::TwoPair(sorted[0].0, sorted[1].0),
            4 if sorted[0].1 == 2 => Self::OnePair(sorted[0].0),
            5 => Self::HighCard,
            n => {
                println!("{n}");
                unreachable!();
            }
        }
    }

    fn strength(&self) -> usize {
        match self {
            Rules::FiveOfKind(_) => 6,
            Rules::FourOfKind(_) => 5,
            Rules::FullHouse(_, _) => 4,
            Rules::ThreeOfKind(_) => 3,
            Rules::TwoPair(_, _) => 2,
            Rules::OnePair(_) => 1,
            Rules::HighCard => 0,
        }
    }
}
