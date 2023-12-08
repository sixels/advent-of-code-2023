use nom::{
    bytes::complete::{take_until, take_while1},
    IResult,
};

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
        todo!()
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
        fn take_number(input: &str) -> IResult<&str, &str> {
            take_while1(|c: char| c.is_ascii_digit())(input)
        }

        // Card N
        let input = &input[4..];
        let (input, _) = take_while1(|c: char| c == ' ')(input)?;
        let (input, number_str) = take_while1(|c: char| c.is_ascii_digit())(input)?;

        // skip ': '
        let input = &input[2..];

        let (input, winning_input) = take_until("|")(input)?;
        let winning = winning_input
            .split(' ')
            .filter_map(|num| take_number(num).ok().map(|(_, s)| s.parse().unwrap()))
            .collect();

        let have = input
            .split(' ')
            .filter_map(|num| take_number(num).ok().map(|(_, s)| s.parse().unwrap()))
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
