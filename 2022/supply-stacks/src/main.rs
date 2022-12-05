#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io::{self, Read, Write};
use std::str::FromStr;

use regex::Regex;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut matrix: [String; 9] = Default::default();
    for line in input.lines().take(8) {
        for (ind, i) in (0..line.len()).step_by(4).enumerate() {
            let crate_id =&line[i+1..i+2];
            if crate_id.contains(char::is_whitespace) {
                continue;
            }
            matrix[ind].push_str(crate_id);
        }
    }

    part1(&input, matrix.clone())?;
    part2(&input, matrix.clone())?;
    Ok(())
}

fn part1(input: &str, mut matrix: [String; 9]) -> Result<()> {
    for line in input.lines().skip(10) {
        let op = Op::from_str(line)?;
        for _ in 0..op.quantity {
            let char_from = matrix[op.from].remove(0);
            matrix[op.to].insert(0, char_from);
        }
    }

    for i in 0..9 {
        write!(io::stdout(), "{}", &matrix[i][0..1])?;
    }
    write!(io::stdout(), "{}", "\n")?;
    Ok(())
}

fn part2(input: &str, mut matrix: [String; 9]) -> Result<()> {
    for line in input.lines().skip(10) {
        let op = Op::from_str(line)?;

        let mut crates_load = String::new();
        for _ in 0..op.quantity {
            let x = matrix[op.from].remove(0);
            crates_load.push(x);
        }
        matrix[op.to].insert_str(0, &crates_load);
    }

    for i in 0..9 {
        write!(io::stdout(), "{}", &matrix[i][0..1])?;
    }
    write!(io::stdout(), "{}", "\n")?;
    Ok(())
}

#[derive(Debug)]
struct Op {
    quantity: i32,
    from: usize,
    to: usize
}

impl FromStr for Op {
    type Err = Box<dyn ::std::error::Error>;

    fn from_str(s: &str) -> Result<Op> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "move ([0-9]+) from ([0-9]) to ([0-9])"
            ).unwrap();
        }

        let captures = match RE.captures(s) {
            None => return Err("unrecognized dependency")?,
            Some(captures) => captures,
        };

        let from: usize = captures[2].parse()?;
        let to: usize = captures[3].parse()?;

        Ok(Op {
            quantity: captures[1].parse()?,
            from: from - 1,
            to: to - 1
        })
    }
}