use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> io::Result<()> {
    let digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut s = 0;
    for i in 0..digits.len() {
        if digits[i] == digits[(i+1)%digits.len()] {
            s += digits[i];
        }
    }

    writeln!(io::stdout(), "{}", s)?;
    Ok(())
}

fn part2(input: &str) -> io::Result<()> {
    let digits: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let mut s = 0;
    for i in 0..digits.len() {
        if digits[i] == digits[(i + (digits.len() / 2)) % digits.len()] {
            s += digits[i];
        }
    }

    writeln!(io::stdout(), "{}", s)?;
    Ok(())
}