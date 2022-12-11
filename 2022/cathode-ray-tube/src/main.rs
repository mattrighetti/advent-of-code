use std::collections::HashSet;
use std::io::{self, Read, Write};
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let cpu = Cpu::from_str(input);

    let tot: i32 = cpu.into_iter()
        .filter(|(x, _)| x == &20 || x == &60 || x == &100 || x == &140 || x == &180 || x == &220)
        .map(|(x, y)| x * y)
        .sum();

    writeln!(io::stdout(), "{:?}", tot)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let cpu = Cpu::from_str(input);

    let mut pixels = Vec::new();
    let mut crt = 0;

    for (_, register) in cpu {
        let sprite: HashSet<i32> = vec![register-1, register, register+1].into_iter().collect();

        if sprite.contains(&(crt % 40)) {
            pixels.push('#');
        } else {
            pixels.push('.');
        }

        crt += 1;
    }

    for chunk in pixels.chunks(pixels.len() / 6) {
        writeln!(io::stdout(), "{}", chunk.into_iter().collect::<String>())?;
    }
    Ok(())
}

#[derive(Debug, Clone)]
enum Op {
    Noop,
    Addx(i32)
}

impl FromStr for Op {
    type Err = Box<dyn ::std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let sp: Vec<&str> = s.split(' ').collect();

        if sp.len() == 1 {
            return Ok(Op::Noop);
        }

        Ok(Op::Addx(sp[1].parse()?))
    }
}

#[derive(Debug, Clone)]
struct Cpu {
    cycle: i32,
    register: i32,
    pipeline: Option<(Op, i32)>,
    ops: Vec<Op>
}

impl Cpu {
    fn from_str(ops: &str) -> Self {
        // Reverse ops to use pop() later
        let ops = ops.lines().filter_map(|line| Op::from_str(line).ok()).rev().collect();

        Cpu { cycle: 0, register: 1, pipeline: None, ops }
    }
}

impl Iterator for Cpu {
    type Item = (i32, i32);

    // Each iteration executes a cycle
    // Ops:
    //   1. Increment cycle
    //   2. Check if there's another op in pipeline
    //     2a. If remaining cycle is zero then we have to set
    //         the register's new value and load next operation
    //     2b. If it's not zero then we need to do nothing other
    //         than decrementing op cycles in pipeline
    //   3. Load new op: If it's a nop then it's executed in the current cycle
    //      if it's a `addx` op then a cycle had just executed,
    //      one remains, put it in the pipeline
    fn next(&mut self) -> Option<Self::Item> {
        self.cycle += 1;

        if let Some((Op::Addx(val), remaining_cycles)) = self.pipeline {
            if remaining_cycles == 0 {
                self.register = self.register + val;
            } else {
                self.pipeline = Some((Op::Addx(val), remaining_cycles - 1));
                return Some((self.cycle, self.register));
            }
        }

        // If pipeline is empty, then check next instruction
        match self.ops.pop() {
            Some(Op::Noop) => {
                // Run in this cycle
                self.pipeline = None;
            },
            Some(Op::Addx(val)) => {
                // This is the first cycle, must repeat another one
                self.pipeline = Some((Op::Addx(val), 1))
            },
            None => {
                if self.pipeline.is_some() {
                    self.pipeline = None;
                    return Some((self.cycle, self.register))
                }

                return None
            }
        }

        Some((self.cycle, self.register))
    }
}