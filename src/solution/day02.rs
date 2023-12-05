use super::{Input, Solution};

pub struct Day2;

impl Solution<2> for Day2 {
    fn solve_part_one(&self, input: &Input) {
        let solution = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(Game::parse_line)
            .filter_map(|game| {
                game.sets
                    .iter()
                    .all(|cubes| cubes.red <= 12 && cubes.green <= 13 && cubes.blue <= 14)
                    .then_some(game.id)
            })
            .sum::<usize>();

        println!("{solution}")
    }

    fn solve_part_two(&self, input: &Input) {
        let solution = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(Game::parse_line)
            .map(|game| {
                let max_cubes = game
                    .sets
                    .iter()
                    .copied()
                    .fold(Cubes::default(), |acc, set| Cubes {
                        red: acc.red.max(set.red),
                        green: acc.green.max(set.green),
                        blue: acc.blue.max(set.blue),
                    });

                max_cubes.red * max_cubes.green * max_cubes.blue
            })
            .sum::<usize>();

        println!("{solution}")
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

struct Game {
    pub id: usize,
    pub sets: Vec<Cubes>,
}

impl Game {
    pub fn parse_line(line: String) -> Self {
        let game_str = line.split(':').collect::<Vec<_>>();
        let id = {
            let id_str = game_str[0]
                .chars()
                .skip(5)
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>();
            id_str.parse().unwrap()
        };

        let sets = {
            game_str[1]
                .split(';')
                .map(|set| {
                    let mut cubes = Cubes::default();
                    for (n, color) in set.split(',').map(|cubes| {
                        let n: usize = cubes
                            .chars()
                            .skip(1)
                            .take_while(|c| c.is_ascii_digit())
                            .collect::<String>()
                            .parse()
                            .unwrap();
                        let color = cubes
                            .chars()
                            .skip_while(|c| !c.is_alphabetic())
                            .collect::<String>();
                        (n, color)
                    }) {
                        match color.as_str() {
                            "red" => cubes.red += n,
                            "green" => cubes.green += n,
                            "blue" => cubes.blue += n,
                            _ => {}
                        }
                    }
                    cubes
                })
                .collect::<Vec<_>>()
        };

        Self { id, sets }
    }
}
