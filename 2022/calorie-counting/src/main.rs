use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

// https://adventofcode.com/2022/day/1

fn part1(input: &str) -> Result<()> {
    let mut max_cal = 0;
    
    let mut c = 0;
    for line in input.lines() {
        if line.is_empty() {
            if c > max_cal {
                max_cal = c;
            }
            c = 0;
            continue;
        }

        let val: i32 = line.parse()?;
        c += val;
    }

    writeln!(io::stdout(), "{}", max_cal)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut calories: Vec<i32> = vec![0];
    let mut index: usize = 0;

    for line in input.lines() {
        if line.is_empty() {
            calories.push(0);
            index += 1;
            continue;
        }

        let c: i32 = line.parse()?;
        calories[index] += c;
    }

    calories.sort_by(|x, y| y.cmp(x));

    writeln!(io::stdout(), "{}", calories.iter().take(3).sum::<i32>())?;
    Ok(())
}