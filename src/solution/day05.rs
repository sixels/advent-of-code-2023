use std::iter;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till, take_until, take_until1},
    combinator::rest,
    multi::separated_list1,
    sequence::pair,
    AsBytes, IResult,
};

use crate::parse::take_number;

use super::{Input, Solution};

pub struct Day5;

impl Solution for Day5 {
    fn solve_part_one(&self, input: &Input) {
        let (_, almanac) =
            Almanac::try_parse(std::str::from_utf8(input.content.as_bytes()).unwrap()).unwrap();

        let solution = almanac
            .seeds
            .iter()
            .map(|seed| find_location(&almanac.mappings, *seed))
            .min()
            .unwrap_or(0);

        println!("{solution}");
    }
    fn solve_part_two(&self, input: &Input) {
        let (_, almanac) =
            Almanac::try_parse(std::str::from_utf8(input.content.as_bytes()).unwrap()).unwrap();

        let Almanac { seeds, mappings } = almanac;

        let solution = seeds
            .chunks_exact(2)
            .map(|seed_range| {
                let (start, range) = (seed_range[0], seed_range[1]);
                iter::successors(Some(start), |n| n.checked_add(1))
                    .take(range)
                    .map(|seed| find_location(&mappings, seed))
                    .min()
                    .unwrap_or(usize::MAX)
            })
            .min()
            .unwrap_or(0);

        println!("{solution}");
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<usize>,

    mappings: Vec<Vec<AlmanacMap>>,
}

#[derive(Debug, Clone, Copy, Default)]
struct AlmanacMap {
    pub destination: usize,
    pub source: usize,
    pub range: usize,
}

impl Almanac {
    pub fn try_parse(input: &str) -> IResult<&str, Self> {
        let (input, first_line) = take_until("\n")(input)?;

        // skip 'seeds: '
        let (_, (_, seeds)) =
            pair(take(7usize), separated_list1(tag(" "), take_number))(first_line)?;

        let (input, _) = take(2usize)(input)?;
        let (input, maps) = separated_list1(tag("\n\n"), |input| {
            let (input, section_name): (&str, &str) = take_till(|c: char| c == ' ')(input)?;

            let (input, _) = take_till(|c| c == '\n')(input)?;
            let input = &input[1..];

            let (input, maps) = AlmanacMap::try_parse_lines(input)?;

            Ok((input, (section_name, maps)))
        })(input)?;

        Ok((
            input,
            Self {
                seeds,
                mappings: maps.into_iter().map(|(_, m)| m).collect(),
            },
        ))
    }
}

impl AlmanacMap {
    pub fn try_parse(input: &str) -> IResult<&str, Self> {
        let (input, numbers) = separated_list1(tag(" "), take_number)(input)?;
        Ok((
            input,
            Self {
                destination: numbers[0],
                source: numbers[1],
                range: numbers[2],
            },
        ))
    }

    fn try_parse_lines(input: &str) -> IResult<&str, Vec<AlmanacMap>> {
        let (inputs, maps) = alt((take_until1("\x0a\x0a"), rest))(input)?;
        let (_, ls) = separated_list1(tag("\n"), AlmanacMap::try_parse)(maps)?;
        Ok((inputs, ls))
    }
}

fn find_location(mappings: &[Vec<AlmanacMap>], seed: usize) -> usize {
    mappings.iter().fold(seed, |number, maps| {
        maps.iter()
            .find_map(|map| {
                if number >= map.source && number < (map.source + map.range) {
                    Some(number - map.source + map.destination)
                } else {
                    None
                }
            })
            .unwrap_or(number)
    })
}
