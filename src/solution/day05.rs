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
            .map(|seed| almanac.find_location(*seed))
            .min()
            .unwrap_or(0);

        println!("{solution}");
    }
    fn solve_part_two(&self, input: &Input) {
        todo!("part two")
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<usize>,

    seed_soil: Vec<AlmanacMap>,
    soil_fertilizer: Vec<AlmanacMap>,
    fertilizer_water: Vec<AlmanacMap>,
    water_light: Vec<AlmanacMap>,
    light_temperature: Vec<AlmanacMap>,
    temperature_humidity: Vec<AlmanacMap>,
    humidity_location: Vec<AlmanacMap>,
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

        let almanac = maps.into_iter().fold(
            Self {
                seeds,
                ..Default::default()
            },
            |mut a, (name, maps)| {
                match name {
                    "seed-to-soil" => a.seed_soil = maps,
                    "soil-to-fertilizer" => a.soil_fertilizer = maps,
                    "fertilizer-to-water" => a.fertilizer_water = maps,
                    "water-to-light" => a.water_light = maps,
                    "light-to-temperature" => a.light_temperature = maps,
                    "temperature-to-humidity" => a.temperature_humidity = maps,
                    "humidity-to-location" => a.humidity_location = maps,
                    _ => panic!("{name} is not valid"),
                };
                a
            },
        );

        Ok((input, almanac))
    }

    fn find_location(&self, seed: usize) -> usize {
        let sequence = [
            &self.seed_soil,
            &self.soil_fertilizer,
            &self.fertilizer_water,
            &self.water_light,
            &self.light_temperature,
            &self.temperature_humidity,
            &self.humidity_location,
        ];

        sequence
            .into_iter()
            .fold(seed, |number, maps| find_mapped_number(maps, number))
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

fn find_mapped_number(maps: &[AlmanacMap], number: usize) -> usize {
    maps.iter()
        .find_map(|map| {
            if number >= map.source && number < (map.source + map.range) {
                Some(number - map.source + map.destination)
            } else {
                None
            }
        })
        .unwrap_or(number)
}
