use std::{
    io::{self, Read, Write},
    u64,
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    writeln!(io::stdout(), "{}", part1(&input)?)?;
    writeln!(io::stdout(), "{}", part2(&input)?)?;
    Ok(())
}

fn part1(input: &str) -> io::Result<u64> {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let seeds: Vec<u64> = parts[0]
        .split(' ')
        .skip(1)
        .filter_map(|x| x.parse().ok())
        .collect();

    let mut mappings: Vec<Vec<Map>> = Vec::new();
    for i in (1..parts.len()).rev() {
        let maps: Vec<Map> = parts[i]
            .split('\n')
            .skip(1)
            .flat_map(Map::try_from)
            .collect();
        mappings.push(maps);
    }

    let mut locations: Vec<u64> = Vec::with_capacity(seeds.len());
    for seed in seeds {
        let mut val = seed;
        for i in 0..mappings.len() {
            val = mappings[i].into_range_mapping(&val);
        }
        locations.push(val);
    }

    Ok(locations.into_iter().min().unwrap())
}

fn part2(input: &str) -> io::Result<u64> {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut mappings: Vec<Vec<Map>> = Vec::new();
    for i in 1..parts.len() {
        let maps: Vec<Map> = parts[i]
            .split('\n')
            .skip(1)
            .flat_map(Map::try_from)
            .collect();

        mappings.push(maps);
    }

    Ok(0)
}

#[derive(Debug)]
struct Map {
    src: u64,
    dst: u64,
    rng: u64,
}

impl Map {
    #[cfg(test)]
    fn new(src: u64, dst: u64, rng: u64) -> Self {
        Map { src, dst, rng }
    }
}

impl TryFrom<&str> for Map {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let vals: Vec<&str> = value.split(' ').collect();
        if vals.len() != 3 {
            return Err("not enough arguments".into());
        }

        let dst = vals[0].parse().unwrap();
        let src = vals[1].parse().unwrap();
        let rng = vals[2].parse().unwrap();

        Ok(Map { src, dst, rng })
    }
}

trait IntoRangeMapping {
    fn into_range_mapping(&self, v: &u64) -> u64;
}

impl IntoRangeMapping for Vec<Map> {
    fn into_range_mapping(&self, v: &u64) -> u64 {
        for map in self {
            if (map.src..map.src + map.rng).contains(v) {
                return map.dst + (v - map.src);
            }
        }

        *v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_range_mapping() {
        let rng_map = vec![Map::new(0, 5, 2), Map::new(6, 9, 3)];
        assert_eq!(5, rng_map.into_range_mapping(&0));
        assert_eq!(6, rng_map.into_range_mapping(&1));
        assert_eq!(3, rng_map.into_range_mapping(&3));
        assert_eq!(4, rng_map.into_range_mapping(&4));
        assert_eq!(5, rng_map.into_range_mapping(&5));
        assert_eq!(9, rng_map.into_range_mapping(&6));
        assert_eq!(10, rng_map.into_range_mapping(&7));
        assert_eq!(11, rng_map.into_range_mapping(&8));
        assert_eq!(9, rng_map.into_range_mapping(&9));
        assert_eq!(10, rng_map.into_range_mapping(&10));
    }

    #[test]
    fn test_part1() {
        assert_eq!(35, part1(INPUT).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(46, part2(INPUT).unwrap());
    }
}
