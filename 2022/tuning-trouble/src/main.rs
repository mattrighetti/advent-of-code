use std::io::{self, Read, Write};
use std::collections::HashSet;
use std::io::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let signal: Vec<char> = input.lines().next().unwrap().chars().collect();
    writeln!(io::stdout(), "{}", find_first_seq_unique_char(4, &signal) + 1)?;
    Ok(())
}


fn part2(input: &str) -> Result<()> {
    let signal: Vec<char> = input.lines().next().unwrap().chars().collect();
    writeln!(io::stdout(), "{}", find_first_seq_unique_char(14, &signal) + 1)?;
    Ok(())
}

// Finds position of last element of the sequence of unique char of given size
fn find_first_seq_unique_char(size: i32, input: &Vec<char>) -> i32 {
    let (mut l, mut r): (usize, usize) = (0, 1);

    let mut seen: HashSet<char> = HashSet::new();
    seen.insert(input[l]);

    loop {
        match seen.contains(&input[r]) {
            false => {
                if r - l + 1 == size as usize {
                    break;
                }
                seen.insert(input[r]);
            },
            true => {
                // Found duplicate:
                // 1. Move l to position of seen input[r] value + 1
                // 2. Remove from set each element until position of seen input[r] + 1
                while input[l] != input[r] {
                    seen.remove(&input[l]);
                    l += 1;
                }

                l += 1;
            }
        }
        r += 1;
    }

    r as i32
}
