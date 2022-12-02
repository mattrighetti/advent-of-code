use std::{io::{self, Read, Write}, collections::HashSet};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

// 2 Sum O(n)
fn part1(input: &str) -> Result<()> {
    let mut values: HashSet<i32> = HashSet::new();
    for line in input.lines() {
        let val: i32 = line.parse()?;
        if values.contains(&(2020-val)) {
            writeln!(io::stdout(), "{}", val * (2020 - val))?;
            break;
        }
        values.insert(val);
    }
    
    Ok(())
}

// 3 Sum O(n^2)
fn part2(input: &str) -> Result<()> {
    let mut values: Vec<i32> = Vec::new();
    for line in input.lines() {
        let val: i32 = line.parse()?;
        values.push(val);
    }

    values.sort();

    for i in 0..values.len() - 2 {
        let mut l = i + 1;
        let mut r = values.len() - 1;

        while l < r {
            let res = values[i] + values[l] + values[r]; 
            if res == 2020 {
                writeln!(io::stdout(), "{}", values[i] * values[l] * values[r])?;
                return Ok(());
            } else if res > 2020 {
                r -= 1;
            } else if res < 2020 {
                l += 1;
            }
        }
    }

    Ok(())
}
