pub mod day1;

use std::{fs, path::Path};

pub trait Solution<const DAY: usize> {
    fn new(input: Input) -> Self;

    fn solve_part_one(&self);
    fn solve_part_two(&self);
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

    pub fn lines(&self) -> impl Iterator<Item = String> + '_ {
        self.content
            .split(|byte| *byte == 0x0a)
            .map(|slice| String::from_utf8_lossy(slice).into_owned())
    }
}
