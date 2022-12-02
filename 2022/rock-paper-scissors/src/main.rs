use std::{
    io::{self, Read, Write},
    str::FromStr
};
use std::error::Error;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Draw,
    Win,
    Lose
}

impl FromStr for Outcome {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Outcome> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(format!("cannot init outcome from {}", s))?
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn vs(&self, shape: &Shape) -> Outcome {
        match (self, shape) {
            (Shape::Rock, Shape::Rock) | (Shape::Paper, Shape::Paper) | (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
            (Shape::Rock, Shape::Paper) | (Shape::Paper, Shape::Scissors) | (Shape::Scissors, Shape::Rock) => Outcome::Lose,
            (Shape::Rock, Shape::Scissors) | (Shape::Scissors, Shape::Paper) | (Shape::Paper, Shape::Rock) => Outcome::Win
        }
    }

    fn get_shape_for_outcome(&self, outcome: Outcome) -> Shape {
        match (self, outcome) {
            (_, Outcome::Draw) => self.clone(),
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Rock, Outcome::Lose) => Shape::Scissors,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Paper, Outcome::Lose) => Shape::Rock,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
            (Shape::Scissors, Outcome::Lose) => Shape::Paper,
        }
    }
}

impl FromStr for Shape {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Shape> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(format!("cannot init shape from {}", s))?
        }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut tot_score = 0;
    for line in input.lines() {
        let (u1, u2) = (Shape::from_str(&line[..1]).unwrap(), Shape::from_str(&line[2..]).unwrap());

        match u2 {
            Shape::Rock => tot_score += 1,
            Shape::Paper => tot_score += 2,
            Shape::Scissors => tot_score += 3
        }

        match u2.vs(&u1) {
            Outcome::Lose => {},
            Outcome::Draw => tot_score += 3,
            Outcome::Win => tot_score += 6
        }
    }

    writeln!(io::stdout(), "{}", tot_score)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut tot_score = 0;
    for line in input.lines() {
        let (u1, o) = (Shape::from_str(&line[..1]).unwrap(), Outcome::from_str(&line[2..]).unwrap());
        let u2 = u1.get_shape_for_outcome(o);

        match u2 {
            Shape::Rock => tot_score += 1,
            Shape::Paper => tot_score += 2,
            Shape::Scissors => tot_score += 3
        }

        match o {
            Outcome::Lose => {},
            Outcome::Draw => tot_score += 3,
            Outcome::Win => tot_score += 6
        }
    }

    writeln!(io::stdout(), "{}", tot_score)?;
    Ok(())
}