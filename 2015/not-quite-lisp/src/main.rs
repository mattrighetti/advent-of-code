use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

// https://adventofcode.com/2015/day/1

fn part1(input: &str) -> io::Result<()> {
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
    }

    writeln!(std::io::stdout(), "{}", floor)?;
    Ok(())
}

fn part2(input: &str) -> io::Result<()> {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {},
        }

        if floor == -1 {
            writeln!(std::io::stdout(), "{}", i + 1)?;
            return Ok(())
        }
    }

    Ok(())
}