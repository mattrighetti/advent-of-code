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
    let mut tot_overlapping = 0;
    for line in input.lines() {
        let comma = match line.find(",") {
            Some(i) => i,
            None => Err("cannot find comma")?
        };
        let (sr1, sr2) = (SectionRange::from_str(&line[..comma])?, SectionRange::from_str(&line[comma+1..])?);

        match (sr1.overlap_completely(&sr2), sr2.overlap_completely(&sr1)) {
            (false, false) => continue,
            _ => tot_overlapping += 1 
        }
    }

    writeln!(io::stdout(), "{}", tot_overlapping)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut tot_overlapping = 0;
    for line in input.lines() {
        let comma = match line.find(",") {
            Some(i) => i,
            None => Err("cannot find comma")?
        };
        let (sr1, sr2) = (SectionRange::from_str(&line[..comma])?, SectionRange::from_str(&line[comma+1..])?);

        match (sr1.overlap(&sr2), sr2.overlap(&sr1)) {
            (false, false) => continue,
            _ => tot_overlapping += 1 
        }
    }

    writeln!(io::stdout(), "{}", tot_overlapping)?;
    Ok(())
}



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SectionRange {
    start: u32,
    end: u32
}

impl SectionRange {
    // To completely overlap, lhs range must be entirely
    // contained in current section range
    fn overlap_completely(&self, lhs: &SectionRange) -> bool {
        self.start <= lhs.start &&
        self.start <= lhs.end &&
        self.end >= lhs.end &&
        self.end >= lhs.start
    }

    // To overlap at all, lhs range must be, at least, partially
    // contained in current section range
    fn overlap(&self, lhs: &SectionRange) -> bool {
        self.start <= lhs.start && self.end >= lhs.start ||
        self.start <= lhs.end && self.end >= lhs.end
    }
}

impl FromStr for SectionRange {
    type Err = Box<dyn ::std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let comma = match s.find("-") {
            Some(i) => i,
            None => Err("cannot find -")?
        };

        let (range1, range2) = (&s[..comma], &s[comma+1..]);
        
        Ok(SectionRange { start: range1.parse()?, end: range2.parse()? })
    }
}