use std::io::{self, Read, Write};
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut tot_area = 0;
    for line in input.lines() {
        let square = Square::from_str(line).unwrap();
        tot_area += square.area() + square.min_surface_area();
    }

    writeln!(io::stdout(), "{}", tot_area)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut tot_feet = 0;
    for line in input.lines() {
        let square = Square::from_str(line).unwrap();
        tot_feet += square.shortest_distance() + square.smallest_perimeter();
    }

    writeln!(io::stdout(), "{}", tot_feet)?;
    Ok(())
}

#[derive(Clone, Copy, Debug)]
struct Square {
    length: i32,
    width: i32,
    height: i32
}

impl Square {
    fn area(&self) -> i32 {
        (2 * self.length * self.width) + (2 * self.width * self.height) + (2 * self.height * self.length)
    }

    fn min_surface_area(&self) -> i32 {
        let mut dims: Vec<i32> = vec![self.length, self.width, self.height];
        dims.sort();

        dims[0] * dims[1]
    }

    fn smallest_perimeter(&self) -> i32 {
        let mut dims = vec![self.length, self.width, self.height];
        dims.sort();

        (dims[0] * 2) + (dims[1] * 2)
    }

    fn shortest_distance(&self) -> i32 {
        [self.length, self.width, self.height].into_iter()
            .reduce(|a, b| a * b)
            .unwrap()
    }
}

impl FromStr for Square {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Square> {
        let vals: Vec<&str> = s.split("x").collect();
        
        Ok(
            Square {
                length: vals[0].parse()?,
                width: vals[1].parse()?,
                height: vals[2].parse()?
            }
        )
    }
}