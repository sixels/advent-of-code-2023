use std::env;

use aoc2023::solution::{day1::Day1, Input, Solution};

fn main() {
    let input_path = env::args()
        .nth(1)
        .expect("You should pass the input file path as argument");
    let input = Input::read(input_path).unwrap();

    let day_one = Day1::new(input);

    day_one.solve_part_two();
}
