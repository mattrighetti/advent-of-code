use std::collections::HashSet;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn get_priority(letter: char) -> u32 {
    match letter.is_uppercase() {
        true => (letter as u32) - ('A' as u32) + 27,
        false => (letter as u32) - ('a' as u32) + 1
    }
}

fn part1(input: &str) -> Result<()> {
    let mut tot_sum = 0;
    let mut seen: HashSet<u32> = HashSet::new();
    for line in input.lines() {
        let (comp1, comp2) = (&line[..line.len()/2], &line[line.len()/2..]);
        
        for c in comp1.chars() {
            let priority = get_priority(c);
            seen.insert(priority);
        }
        
        for c in comp2.chars() {
            let priority = get_priority(c);
            if seen.contains(&priority) {
                tot_sum += priority;
                break;
            }
        }

        seen.clear();
    }

    writeln!(io::stdout(), "{}", tot_sum)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut tot_sum = 0;
    let lines: Vec<&str> = input.lines().into_iter().collect();

    for i in (0..lines.len() - 3).step_by(3) {
        let a: HashSet<char> = lines[i].chars().collect();
        let b: HashSet<char> = lines[i+1].chars().collect();
        let c: HashSet<char> = lines[i+2].chars().collect();

        for v in a {
            if b.contains(&v) && c.contains(&v) {
                tot_sum += get_priority(v);
            }
        }
    }

    writeln!(io::stdout(), "{}", tot_sum)?;
    Ok(())
}