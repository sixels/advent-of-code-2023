use std::collections::BTreeMap;

use nom::{
    bytes::complete::{take_till1, take_while1},
    character::complete::multispace1,
    multi::separated_list1,
    IResult,
};

use crate::parse::take_number;

use super::{Input, Solution};

pub struct Day6;

impl Solution for Day6 {
    fn solve_part_one(&self, input: &Input) {
        let input_text = std::str::from_utf8(input.content.as_slice()).unwrap();
        let (_, sheet) = Sheet::try_parse(input_text).unwrap();

        let solution: usize = sheet.solve();
        println!("{solution}");
    }

    fn solve_part_two(&self, input: &Input) {
        let input_text = std::str::from_utf8(input.content.as_slice()).unwrap();
        let (_, sheet) = Sheet::try_parse_single(input_text).unwrap();

        let solution: usize = sheet.solve();
        println!("{solution}");
    }
}

#[derive(Debug)]
struct Sheet {
    time_distance: BTreeMap<usize, usize>,
}

impl Sheet {
    pub fn try_parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = take_till1(|c: char| c.is_ascii_digit())(input)?;
        let (input, times) = separated_list1(multispace1, take_number)(input)?;

        let (input, _) = take_till1(|c: char| c.is_ascii_digit())(input)?;
        let (input, distances) = separated_list1(multispace1, take_number)(input)?;

        let time_distance = times.into_iter().zip(distances).collect();
        Ok((input, Self { time_distance }))
    }

    fn try_parse_single(input: &str) -> IResult<&str, Self> {
        let (input, _) = take_till1(|c: char| c.is_ascii_digit())(input)?;
        let (input, all_times) = take_till1(|c| c == '\n')(input)?;
        let time = all_times.replace(' ', "").parse().unwrap();

        let (input, _) = take_till1(|c: char| c.is_ascii_digit())(input)?;
        let (input, all_distances) = take_while1(|c: char| c != '\n' && c != '\0')(input)?;
        let distance = all_distances.replace(' ', "").parse().unwrap();

        let mut time_distance = BTreeMap::new();
        time_distance.insert(time, distance);
        Ok((input, Self { time_distance }))
    }

    fn solve(self) -> usize {
        self.time_distance
            .into_iter()
            .map(|(time, distance)| {
                let ft = time as f64;
                let fd = distance as f64;

                (
                    (-ft + (ft.powi(2) - 4.0 * fd).sqrt()) / -2.0,
                    (-ft - (ft.powi(2) - 4.0 * fd).sqrt()) / -2.0,
                )
            })
            .map(|(l, r)| (r.ceil() as usize - 1) - (l.floor() as usize + 1) + 1)
            .reduce(|acc, a| acc * a)
            .unwrap_or(0)
    }
}
