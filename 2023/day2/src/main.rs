use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    writeln!(io::stdout(), "{}", part1(&input)?)?;
    writeln!(io::stdout(), "{}", part2(&input)?)?;
    Ok(())
}

fn part1(input: &str) -> io::Result<u32> {
    let config: RGB = (12, 13, 14);

    let sum = input
        .lines()
        .map(Game::from)
        .filter(|x| {
            let rgb = x.get_max_rgb_value();
            rgb.0 <= config.0 && rgb.1 <= config.1 && rgb.2 <= config.2
        })
        .map(|x| x.id)
        .sum();

    Ok(sum)
}

fn part2(input: &str) -> io::Result<u32> {
    let sum = input
        .lines()
        .map(Game::from)
        .map(|x| x.get_max_rgb_value())
        .map(|(r, g, b)| r * g * b)
        .sum();

    Ok(sum)
}

type RGB = (u32, u32, u32);

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<RGB>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game_str, sets_str) = value.split_once(":").unwrap();
        let id = game_str.split_once(" ").unwrap().1.parse().unwrap();

        let mut sets = Vec::new();

        for set in sets_str.split(";") {
            let mut rgb = (0, 0, 0);
            for hand in set.split(",") {
                match hand.trim().split_once(" ").unwrap() {
                    (id, "red") => {
                        rgb.0 = id.parse().unwrap();
                    }
                    (id, "green") => {
                        rgb.1 = id.parse().unwrap();
                    }
                    (id, "blue") => {
                        rgb.2 = id.parse().unwrap();
                    }
                    _ => {}
                }
            }
            sets.push(rgb);
        }

        Game { id, sets }
    }
}

impl Game {
    fn get_max_rgb_value(&self) -> RGB {
        let mut max_rgb = (0, 0, 0);
        for (r, g, b) in &self.sets {
            if max_rgb.0 < *r {
                max_rgb.0 = *r;
            }
            if max_rgb.1 < *g {
                max_rgb.1 = *g;
            }
            if max_rgb.2 < *b {
                max_rgb.2 = *b;
            }
        }

        max_rgb
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_parse() {
        let g = Game::from("Game 123: 2 green, 1 red; 3 red, 4 blue");
        assert_eq!(g.id, 123);
        assert_eq!(g.sets, vec![(1, 2, 0), (3, 0, 4)]);
    }

    #[test]
    fn test_max_rgb_value() {
        let g1 = Game::from("Game 123: 2 green, 1 red; 3 red, 4 blue");
        assert_eq!(g1.get_max_rgb_value(), (3, 2, 4));
        let g2 = Game::from("Game 123: 1 red; 3 red, 4 blue");
        assert_eq!(g2.get_max_rgb_value(), (3, 0, 4));
        let g3 = Game::from("Game 123: 2 green");
        assert_eq!(g3.get_max_rgb_value(), (0, 2, 0));
    }

    #[test]
    fn test_part1() {
        assert_eq!(0, part1("Game 3: 13 red").unwrap());
        assert_eq!(0, part1("Game 3: 14 green").unwrap());
        assert_eq!(0, part1("Game 3: 15 blue").unwrap());
        assert_eq!(1, part1("Game 1: 10 green; 5 blue").unwrap());
        assert_eq!(1, part1("Game 1: 10 green; 5 blue").unwrap());
        assert_eq!(1, part1("Game 1: 10 green; 5 blue").unwrap());
        assert_eq!(2, part1("Game 2: 12 red").unwrap());
        assert_eq!(2, part1("Game 2: 13 green").unwrap());
        assert_eq!(2, part1("Game 2: 14 blue").unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            2286,
            part2(
                r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )
            .unwrap()
        );
    }
}
