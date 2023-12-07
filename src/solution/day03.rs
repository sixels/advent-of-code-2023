use std::collections::HashMap;

use super::{Input, Solution};

pub struct Day3;

const LOOKUP: [(isize, isize); 8] = [
    // top
    (-1, -1),
    (-1, 0),
    (-1, 1),
    // left/right
    (0, -1),
    (0, 1),
    // bottom
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Solution<3> for Day3 {
    fn solve_part_one(&self, input: &Input) {
        let grid = Grid::new(&input.content);

        let symbols = grid.lines.iter().enumerate().flat_map(|(y, line)| {
            line.char_indices()
                .filter_map(|(x, c)| is_symbol(c).then_some((x, y)))
                .collect::<Vec<_>>()
        });

        let solution: u32 = symbols
            .fold(HashMap::new(), |acc: HashMap<_, _>, (sym_x, sym_y)| {
                LOOKUP
                    .iter()
                    .filter_map(|&(dx, dy)| {
                        let Some(x) = sym_x.checked_add_signed(dx) else {
                            return None;
                        };
                        let Some(y) = sym_y.checked_add_signed(dy) else {
                            return None;
                        };

                        if y >= grid.lines.len() || x >= grid.lines[y].len() {
                            return None;
                        }

                        grid.lines
                            .get(y)
                            .map(|line| (line.chars().nth(x).unwrap(), (x, y)))
                    })
                    .fold(acc, |mut acc, (ch, (x, y))| {
                        if !ch.is_ascii_digit() {
                            return acc;
                        }

                        let (num_start, num) = get_number(&grid.lines[y], x);

                        acc.insert((num_start, y), num);

                        acc
                    })
            })
            .values()
            .sum();
        println!("{solution}")
    }

    fn solve_part_two(&self, input: &Input) {
        todo!()
    }
}

struct Grid {
    lines: Vec<String>,
}

impl Grid {
    pub fn new(data: &[u8]) -> Self {
        let lines = data
            .split(|b| *b == b'\n')
            .map(|l| String::from_utf8_lossy(l).into_owned())
            .collect::<Vec<String>>();

        Self { lines }
    }
}

fn is_symbol(c: char) -> bool {
    c != '.' && c.is_ascii_punctuation()
}

fn get_number(line: &str, x: usize) -> (usize, u32) {
    assert!(line.chars().nth(x).unwrap().is_ascii_digit());

    let start = line
        .char_indices()
        .rev()
        .skip(line.len() - x - 1)
        .find_map(|(i, c)| (!c.is_ascii_digit()).then_some(i + 1))
        .unwrap_or_default();

    let end = line
        .char_indices()
        .skip(x + 1)
        .find_map(|(i, c)| (!c.is_ascii_digit()).then_some(i))
        .unwrap_or(line.len());

    (start, line[start..end].parse().unwrap())
}
