use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    writeln!(io::stdout(), "{}", part1(&input)?)?;
    writeln!(io::stdout(), "{}", part2(&input)?)?;
    Ok(())
}

fn part1(input: &str) -> io::Result<u32> {
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let ms: Vec<f64> = lines[0]
        .split(" ")
        .skip(1)
        .filter(|x| !x.is_empty())
        .flat_map(|x| x.parse())
        .collect();

    let records: Vec<f64> = lines[1]
        .split(" ")
        .skip(1)
        .filter(|x| !x.is_empty())
        .flat_map(|x| x.parse())
        .collect();

    let mut eqs = Vec::new();
    for i in 0..ms.len() {
        eqs.push(Eq::new(1.0, -ms[i], records[i]));
    }

    let res = eqs
        .into_iter()
        .map(|x| x.range())
        .map(|(lower, upper)| upper - lower + 1)
        .reduce(|acc, e| acc * e)
        .unwrap();

    Ok(res as u32)
}

fn part2(input: &str) -> io::Result<u32> {
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let ms: f64 = lines[0]
        .split(" ")
        .skip(1)
        .filter(|x| !x.is_empty())
        .collect::<String>()
        .parse()
        .unwrap();

    let record: f64 = lines[1]
        .split(" ")
        .skip(1)
        .filter(|x| !x.is_empty())
        .collect::<String>()
        .parse()
        .unwrap();

    let eq = Eq::new(1.0, -ms, record);
    let (lower, upper) = eq.range();

    Ok(upper as u32 - lower as u32 + 1)
}

struct Eq {
    a: f64,
    b: f64,
    c: f64,
}

impl Eq {
    fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    fn range(&self) -> (i32, i32) {
        let discriminant = self.b * self.b - 4.0 * self.a * self.c;

        if discriminant >= 0.0 {
            let mut root1 = (-self.b + discriminant.sqrt()) / (2.0 * self.a);
            let mut root2 = (-self.b - discriminant.sqrt()) / (2.0 * self.a);

            if root1.fract() == 0.0 {
                root1 -= 1.0;
            }

            if root2.fract() == 0.0 {
                root2 += 1.0;
            }

            return (root2.ceil() as i32, root1.floor() as i32);
        }

        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            (4 * 8 * 9),
            part1("Time:      7  15   30\nDistance:  9  40  200").unwrap()
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            71503,
            part2("Time:      7  15   30\nDistance:  9  40  200").unwrap()
        )
    }
}
