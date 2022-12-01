use std::{io::{self, Read, Write}, collections::HashSet};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut freq = 0;
    for line in input.lines() {
        let change: i32 = line.parse()?;
        freq += change;
    }

    writeln!(io::stdout(), "{}", freq)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut change_list = Vec::new();
    for line in input.lines() {
        let change: i32 = line.parse()?;
        change_list.push(change);
    }

    let mut i = 0;
    let mut freq = 0;
    let mut frequencies: HashSet<i32> = HashSet::new();
    loop {
        freq += change_list[i];
        if frequencies.contains(&freq) {
            writeln!(io::stdout(), "{}", freq)?;
            break;
        }
        frequencies.insert(freq);
        i = (i + 1) % change_list.len();
    }

    Ok(())
}