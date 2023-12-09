use std::env;

use aoc2023::solution::{
    day01::Day1, day02::Day2, day03::Day3, day04::Day4, day05::Day5, day06::Day6, Input, Solution,
};

fn main() {
    let days: Vec<Box<dyn Solution>> = vec![
        Box::new(Day1),
        Box::new(Day2),
        Box::new(Day3),
        Box::new(Day4),
        Box::new(Day5),
        Box::new(Day6),
    ];

    let day_option = env::args()
        .nth(1)
        .expect("You should pass the day number as first argument");

    let day_number: usize = day_option
        .parse()
        .expect("The day number should be a number greater than 0");
    assert!(day_number > 0);

    let input_option = env::args()
        .nth(2)
        .expect("You should pass the input file path as second argument");

    let input = Input::read(input_option).unwrap();

    if let Some(challenge) = days.get(day_number - 1) {
        challenge.solve_part_one(&input);
        challenge.solve_part_two(&input);
    }
}
