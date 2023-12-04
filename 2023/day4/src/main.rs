use std::{
    collections::HashSet,
    io::{self, Read, Write},
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    writeln!(io::stdout(), "{}", part1(&input)?)?;
    writeln!(io::stdout(), "{}", part2(&input)?)?;
    Ok(())
}

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (_, all_nums) = value.split_once(":").unwrap();
        let (winning_str, numbers_str) = all_nums.split_once("|").unwrap();

        let winning: HashSet<u32> = winning_str
            .trim()
            .replace("  ", " ")
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();

        let numbers: HashSet<u32> = numbers_str
            .trim()
            .replace("  ", " ")
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();

        Card { winning, numbers }
    }
}

impl Card {
    fn matching(&self) -> u32 {
        let mut matching = 0;
        for w in &self.winning {
            if self.numbers.contains(w) {
                matching += 1;
            }
        }

        matching
    }

    fn points(&self) -> u32 {
        let mut matching = 0;
        for w in &self.winning {
            if self.numbers.contains(w) {
                matching += 1;
            }
        }

        if matching == 0 {
            return 0;
        }

        1 << (matching - 1)
    }
}

fn part1(input: &str) -> io::Result<u32> {
    let points = input.lines().map(Card::from).map(|x| x.points()).sum();

    Ok(points)
}

fn part2(input: &str) -> io::Result<u32> {
    let points: Vec<u32> = input
        .lines()
        .map(Card::from)
        .map(|x| x.matching())
        .collect();

    let mut cards: Vec<u32> = vec![1; points.len()];

    for (i, point) in points.iter().enumerate() {
        let index = i as u32 + 1;
        let incr = cards[i];

        for l in index..index + point {
            if let Some(v) = cards.get_mut(l as usize) {
                *v += incr;
            }
        }
    }

    Ok(cards.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            13,
            part1(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            30,
            part2(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            )
            .unwrap()
        );
    }
}
