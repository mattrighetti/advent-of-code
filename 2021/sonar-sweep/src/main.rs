use std::{path::Path, io::{BufReader, BufRead}, fs::File};

fn main() {
    let input_vec = read_vec_input(Path::new("input"));

    // Part One
    let mut measurements = count_measurements(input_vec.clone());
    println!("Part one: {}", measurements);

    // Part Two
    let sliding_window_input = sliding_window_vec(input_vec);
    measurements = count_measurements(sliding_window_input);
    println!("Part two: {}", measurements);
}

fn count_measurements(input: Vec<i64>) -> i32 {
    let mut counter = 0;
    let mut prev: Option<i64> = None;
    for num in input {
        if prev.is_some() && num > prev.unwrap() {
            counter += 1;
        }

        prev = Some(num);
    }

    counter
}

fn sliding_window_vec(input: Vec<i64>) -> Vec<i64> {
    let mut sliding_window = Vec::new();
    for i in 0..(input.len() - 2) {
        sliding_window.push(
            input.get(i).unwrap() + input.get(i + 1).unwrap() + input.get(i + 2).unwrap()
        );
    }

    sliding_window
}

fn read_vec_input(input_path: &Path) -> Vec<i64> {
    let file = File::open(&input_path).ok().unwrap();
    let file_reader = BufReader::new(file);
    let mut vector: Vec<i64> = Vec::new();

    for line in file_reader.lines() {
        match line {
            Err(err) => panic!("encountered error: {:?}", err),
            Ok(line_string) => match line_string.trim().parse::<i64>() {
                Ok(number) => vector.push(number),
                Err(_) => panic!("cannot parse number: {:?}", line_string)
            }
        }
    }

    vector
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_corner() {
        let input = vec![10, 10, 10, 10, 10];
        assert_eq!(0, count_measurements(input));
    }

    #[test]
    fn test_aoc() {
        let input = vec![199,200,208,210,200,207,240,269,260,263];
        assert_eq!(7, count_measurements(input));
    }

    #[test]
    fn test_corner2() {
        let input = vec![11,12,13,15];
        assert_eq!(3, count_measurements(input));
    }

    #[test]
    fn test_sliding_widow() {
        let input = vec![10, 20, 30, 40];
        assert_eq!(sliding_window_vec(input), vec![60, 90]);
    }
}

