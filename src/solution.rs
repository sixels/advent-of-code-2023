pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

use std::{fs, path::Path};

pub trait Solution {
    fn solve_part_one(&self, input: &Input);
    fn solve_part_two(&self, input: &Input);
}

pub struct Input {
    /// the content of the input text file
    content: Vec<u8>,
}

impl Input {
    pub fn read(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let content = fs::read(path.as_ref())?;

        Ok(Self { content })
    }

    pub fn lines(&self) -> impl Iterator<Item = &str> + '_ {
        self.content
            .split(|byte| *byte == 0x0a)
            .map(|slice| std::str::from_utf8(slice).unwrap())
    }
}
