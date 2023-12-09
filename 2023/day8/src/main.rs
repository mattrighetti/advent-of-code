use core::panic;
use std::{
    collections::HashMap,
    io::{self, Read, Write},
};

use lcmx::lcmx;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    writeln!(io::stdout(), "{}", part1(&input)?)?;
    writeln!(io::stdout(), "{}", part2(&input)?)?;
    Ok(())
}

fn part1(input: &str) -> io::Result<u32> {
    let (track, map) = input.split_once("\n\n").unwrap();
    let track: Vec<char> = track.chars().collect();
    let mut map = MapModel::from(map);

    let mut i = 0;
    loop {
        match track[i % track.len()] {
            'R' => map.right(),
            'L' => map.left(),
            _ => panic!("invalid move"),
        };

        if map.pos == "ZZZ" {
            return Ok(i as u32 + 1);
        }

        i += 1;
    }
}

fn part2(input: &str) -> io::Result<u64> {
    let (track, map) = input.split_once("\n\n").unwrap();
    let track: Vec<char> = track.chars().collect();
    let mut map = MapModel::from(map);

    let mut starting_pos: Vec<String> = Vec::new();
    for k in map.map.keys().filter(|x| x.ends_with("A")) {
        starting_pos.push(k.to_owned());
    }

    let mut steps: Vec<u64> = Vec::new();

    for sp in starting_pos {
        map.pos = sp;
        let mut i = 0;
        loop {
            match track[i % track.len()] {
                'R' => map.right(),
                'L' => map.left(),
                _ => panic!("invalid move"),
            };

            if map.pos.ends_with("Z") {
                steps.push(i as u64 + 1);
                break;
            }

            i += 1;
        }
    }

    Ok(lcmx(&steps).unwrap())
}

static RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap());

#[derive(Debug)]
struct MapModel {
    pos: String,
    map: HashMap<String, (String, String)>,
}

impl MapModel {
    fn left(&mut self) {
        let (l, _) = self.map.get(&self.pos).unwrap();
        self.pos = l.to_owned();
    }

    fn right(&mut self) {
        let (_, r) = self.map.get(&self.pos).unwrap();
        self.pos = r.to_owned();
    }
}

impl From<&str> for MapModel {
    fn from(value: &str) -> Self {
        let mut map = HashMap::new();
        for line in value.lines() {
            let captures = match RE.captures(&line) {
                None => panic!("cannot parse line"),
                Some(captures) => captures,
            };

            let key = captures[1].to_string();
            let left = captures[2].to_string();
            let right = captures[3].to_string();

            map.insert(key, (left, right));
        }

        MapModel {
            pos: String::from("AAA"),
            map,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_parse() {
        let map = MapModel::from(
            r"AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );

        println!("{:?}", map);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            2,
            part1(
                r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
            )
            .unwrap()
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            6,
            part2(
                r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
            )
            .unwrap()
        )
    }
}
