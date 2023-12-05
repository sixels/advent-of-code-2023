use std::env;

use aoc2023::solution::{day02::Day2, Input, Solution};

fn main() {
    let input_path = env::args()
        .nth(1)
        .expect("You should pass the input file path as argument");
    let input = Input::read(input_path).unwrap();

    Day2.solve_part_two(&input);
}
