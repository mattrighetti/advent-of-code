use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn fuel2(mass: i32) -> i32 {
    let fuel_required = (mass / 3) - 2;
    if fuel_required <= 0 {
        return 0;
    }

    return fuel_required + fuel2(fuel_required);
}

fn part1(input: &str) -> Result<()> {
    let mut tot_fuel = 0;
    for line in input.lines() {
        let mass: i32 = line.parse()?;
        tot_fuel += mass / 3 - 2;        
    }

    writeln!(io::stdout(), "{}", tot_fuel)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut tot_fuel = 0;
    for line in input.lines() {
        let mass: i32 = line.parse()?;
        tot_fuel += fuel2(mass);
    }

    writeln!(io::stdout(), "{}", tot_fuel)?;
    Ok(())
}