use std::{
    cmp,
    io::{self, Read, Write},
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
    for i in 1..parts.len() {
        let maps: Vec<Map> = parts[i]
            .split('\n')
            .skip(1)
            .filter(|x| !x.is_empty())
            .map(Map::from)
            .collect();
        mappings.push(maps);
    }

    let mut locations: Vec<u64> = Vec::with_capacity(seeds.len());
    for seed in seeds {
        let mut value_map = seed;
        for i in 0..mappings.len() {
            value_map = mappings[i].get_mapped_value(&value_map);
        }
        locations.push(value_map);
    }

    Ok(locations.into_iter().min().unwrap())
}

fn part2(input: &str) -> io::Result<u64> {
    let parts: Vec<&str> = input.split("\n\n").collect();

    // tuple indicating start and end values of range
    let mut seed_ranges: Vec<(u64, u64)> = parts[0]
        .split(' ')
        .skip(1)
        .filter_map(|x| x.parse().ok())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|w| (w[0], w[0] + w[1]))
        .collect();

    let mut mappings: Vec<Vec<Map>> = Vec::new();
    for i in 1..parts.len() {
        let mapping: Vec<Map> = parts[i]
            .split('\n')
            .skip(1)
            .filter(|x| !x.is_empty())
            .map(Map::from)
            .collect();

        mappings.push(mapping);
    }

    for range_map in mappings {
        let mut seed_ranges_map: Vec<(u64, u64)> = Vec::new();
        while let Some((start, end)) = seed_ranges.pop() {
            let mut ta = Some((start, end));
            for i in 0..range_map.len() {
                if let Some((left_overlap, right_overlap)) = overlaps(start, end, &range_map[i]) {
                    seed_ranges_map.push((
                        left_overlap - range_map[i].src + range_map[i].dst,
                        right_overlap - range_map[i].src + range_map[i].dst,
                    ));

                    if left_overlap > start {
                        seed_ranges.push((start, left_overlap));
                    }

                    if right_overlap < end {
                        seed_ranges.push((right_overlap, end));
                    }

                    ta = None;
                }
            }

            if ta.is_some() {
                seed_ranges_map.push(ta.unwrap());
            }
        }

        seed_ranges = seed_ranges_map.clone();
    }

    seed_ranges.sort();
    Ok(seed_ranges[0].0)
}

#[derive(Debug, PartialEq, Eq)]
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

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let vals: Vec<&str> = value.split(' ').collect();
        let dst = vals[0].parse().unwrap();
        let src = vals[1].parse().unwrap();
        let rng = vals[2].parse().unwrap();
        Map { src, dst, rng }
    }
}

trait IntoRangeMapping {
    fn get_mapped_value(&self, v: &u64) -> u64;
}

impl IntoRangeMapping for Vec<Map> {
    fn get_mapped_value(&self, v: &u64) -> u64 {
        for map in self {
            if (map.src..map.src + map.rng).contains(v) {
                return map.dst + (v - map.src);
            }
        }

        *v
    }
}

fn overlaps(r_start: u64, r_end: u64, m: &Map) -> Option<(u64, u64)> {
    let left_overlap = cmp::max(r_start, m.src);
    let right_overlap = cmp::min(r_end, m.src + m.rng);

    match left_overlap < right_overlap {
        true => Some((left_overlap, right_overlap)),
        false => None,
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
    fn test_map_parse() {
        assert_eq!(Map::new(20, 30, 40), Map::from("30 20 40"));
    }

    #[test]
    fn test_range_mapping() {
        let rng_map = vec![Map::new(0, 5, 2), Map::new(6, 9, 3)];
        assert_eq!(5, rng_map.get_mapped_value(&0));
        assert_eq!(6, rng_map.get_mapped_value(&1));
        assert_eq!(3, rng_map.get_mapped_value(&3));
        assert_eq!(4, rng_map.get_mapped_value(&4));
        assert_eq!(5, rng_map.get_mapped_value(&5));
        assert_eq!(9, rng_map.get_mapped_value(&6));
        assert_eq!(10, rng_map.get_mapped_value(&7));
        assert_eq!(11, rng_map.get_mapped_value(&8));
        assert_eq!(9, rng_map.get_mapped_value(&9));
        assert_eq!(10, rng_map.get_mapped_value(&10));
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
