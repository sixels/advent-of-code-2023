use super::{Input, Solution};

pub struct Day1 {
    input: Input,
}

impl Solution<1> for Day1 {
    fn new(input: Input) -> Self {
        Self { input }
    }

    fn solve_part_one(&self) {
        let solution: u32 = self
            .input
            .lines()
            .map(|line| {
                let left = line
                    .chars()
                    .find(|c| c.is_ascii_digit())
                    .map(to_ascii_digit)
                    .unwrap_or(0);

                let right = line
                    .chars()
                    .rev()
                    .find(|c| c.is_ascii_digit())
                    .map(to_ascii_digit)
                    .unwrap_or(0);

                left * 10 + right
            })
            // .inspect(|x| println!("{x}"))
            .sum();

        println!("{solution}")
    }

    fn solve_part_two(&self) {
        let solution: u32 = self
            .input
            .lines()
            .map(|line| {
                let left = line
                    .char_indices()
                    .find_map(|(i, c)| try_match_digit(c, &line[i..]))
                    .unwrap_or(0);

                let right = line
                    .char_indices()
                    .rev()
                    .find_map(|(i, c)| try_match_digit(c, &line[i..]))
                    .unwrap_or(0);

                left * 10 + right
            })
            // .inspect(|x| println!("{x}"))
            .sum();

        println!("{solution}")
    }
}

fn try_match_digit(value: char, rest: &str) -> Option<u32> {
    if value.is_ascii_digit() {
        return Some(to_ascii_digit(value));
    }

    match value {
        'o' if rest.len() >= 3 && &rest[..3] == "one" => Some(1),
        't' if rest.len() >= 3 && &rest[..3] == "two" => Some(2),
        't' if rest.len() >= 5 && &rest[..5] == "three" => Some(3),
        'f' if rest.len() >= 4 && &rest[..4] == "four" => Some(4),
        'f' if rest.len() >= 4 && &rest[..4] == "five" => Some(5),
        's' if rest.len() >= 3 && &rest[..3] == "six" => Some(6),
        's' if rest.len() >= 5 && &rest[..5] == "seven" => Some(7),
        'e' if rest.len() >= 5 && &rest[..5] == "eight" => Some(8),
        'n' if rest.len() >= 4 && &rest[..4] == "nine" => Some(9),
        _ => None,
    }
}

fn to_ascii_digit(c: char) -> u32 {
    (c as u32).saturating_sub(48)
}

// fn is_digit(c: )
