use std::{collections::BTreeMap, iter};

use nom::{
    bytes::complete::{take_until, take_while1},
    IResult,
};

use crate::parse::take_number;

use super::Solution;

pub struct Day4;

impl Solution for Day4 {
    fn solve_part_one(&self, input: &super::Input) {
        let solution: u32 = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| Card::parse_line(line).unwrap().1)
            .map(|card| {
                let matches = card
                    .winning
                    .iter()
                    .filter(|a| card.have.contains(*a))
                    .count();

                match matches {
                    0 => 0,
                    1 => 1,
                    _ => 2 << (matches - 2),
                }
            })
            .sum();

        println!("{solution}");
    }

    fn solve_part_two(&self, input: &super::Input) {
        let solution: u32 = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| Card::parse_line(line).unwrap().1)
            .map(|card| {
                let matches = card
                    .winning
                    .iter()
                    .filter(|a| card.have.contains(*a))
                    .count();
                matches
            })
            .enumerate()
            .fold(BTreeMap::new(), |mut acc, (i, matches)| {
                let count = acc.entry(i).or_insert(1);
                iter::repeat(*count)
                    .take(matches)
                    .enumerate()
                    .fold(acc, |mut acc, (j, n)| {
                        let next = acc.entry(i + j + 1).or_insert(1);
                        *next += n;
                        acc
                    })
            })
            .into_values()
            .sum();

        println!("{solution}");
    }
}

pub struct Card {
    #[allow(dead_code)]
    number: usize,
    winning: Vec<u8>,
    have: Vec<u8>,
}
impl Card {
    pub fn parse_line(input: &str) -> IResult<&str, Self> {
        // Card N
        let input = &input[4..];
        let (input, _) = take_while1(|c: char| c == ' ')(input)?;
        let (input, number_str) = take_while1(|c: char| c.is_ascii_digit())(input)?;

        // skip ': '
        let input = &input[2..];

        let (input, winning_input) = take_until("|")(input)?;
        let winning = winning_input
            .split(' ')
            .filter_map(|num| take_number(num).ok().map(|(_, n)| n))
            .collect();

        let have = input
            .split(' ')
            .filter_map(|num| take_number(num).ok().map(|(_, n)| n))
            .collect();

        let number = number_str.parse().unwrap();
        Ok((
            input,
            Self {
                number,
                winning,
                have,
            },
        ))
    }
}
