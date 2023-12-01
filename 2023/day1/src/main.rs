use std::{
    io::{self, Read, Write},
    u32,
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    writeln!(io::stdout(), "{}", part1(&input)?)?;
    writeln!(io::stdout(), "{}", part2(&input)?)?;
    Ok(())
}

fn part1(input: &str) -> io::Result<u32> {
    let sum: u32 = input
        .lines()
        .map(|x| {
            let digits: Vec<u32> = x.chars().flat_map(|x| x.to_digit(10)).collect();
            (digits[0] * 10) + digits[digits.len() - 1]
        })
        .sum();

    Ok(sum)
}

fn part2(input: &str) -> io::Result<u32> {
    let sum: u32 = input.lines().map(repl_digits).sum();

    Ok(sum)
}

fn repl_digits(x: &str) -> u32 {
    let digits = vec![
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("0", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("zero", "0"),
    ];

    let mut first: (&str, usize) = ("", x.len());
    let mut last: (&str, usize) = ("", 0);

    for (substr, digit) in &digits {
        let occ: Vec<_> = x.match_indices(substr).map(|x| x.0).collect();
        if occ.len() == 0 {
            continue;
        }

        let (min, max) = (occ[0], occ[occ.len() - 1]);

        if min <= first.1 {
            first = (digit, min);
        }

        if max >= last.1 {
            last = (digit, max);
        }
    }

    format!("{}{}", first.0, last.0).parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("ad1f3").unwrap(), 13);
        assert_eq!(part1("ad1\na3\n11\n0").unwrap(), 11 + 33 + 11 + 0);
    }

    #[test]
    fn test_repl_digits() {
        assert_eq!(repl_digits("eightwothree"), 83);
        assert_eq!(repl_digits("13eightwothree"), 13);
        assert_eq!(repl_digits("13oneight"), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("ad1f3").unwrap(), 13);
        assert_eq!(part2("ad1\na3\n11\n0").unwrap(), 11 + 33 + 11 + 0);
        assert_eq!(part2("zero").unwrap(), 0);
        assert_eq!(part2("three").unwrap(), 33);
        assert_eq!(part2("1\nthree\nonetwothree2three").unwrap(), 11 + 33 + 13);
        assert_eq!(
            part2(
                r"two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen"
            )
            .unwrap(),
            281
        );
    }
}
