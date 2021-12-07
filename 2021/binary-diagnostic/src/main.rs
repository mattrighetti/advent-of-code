use std::{path::Path, fs::File, io::{BufReader, BufRead}};

fn main() {
    let input_vec = read_vec_input(Path::new("input"));

    // Part One
    let gamma = get_gamma(&input_vec);
    let epsilon = gamma ^ 0b111111111111;

    println!("epsilon: {}, gamma: {}\nresult: {}", epsilon, gamma, epsilon*gamma);

    // Part two
    let co2_value = get_value_with_criteria(&input_vec, |position, input| {
        match most_common_bit(position, input) {
            0 => 1,
            1 => 0,
            _ => 0
        } 
    });
    let oxigen_value = get_value_with_criteria(&input_vec, |position, input| {
        match most_common_bit(position, input) {
            1 => 1,
            0 => 0,
            _ => 1
        } 
    });

    println!("oxygen: {}, co2: {}\nresult: {}", oxigen_value, co2_value, oxigen_value * co2_value);
}

fn get_gamma(vec: &Vec<i32>) -> i32 {
    let mut gamma = 0;

    for i in 0..12 {
        if most_common_bit(i, vec) == 1 {
            gamma += 2_i32.pow(i as u32);
        }
    }

    gamma
}

fn most_common_bit(position: usize, input: &Vec<i32>) -> i32 {
    let mut num_1 = 0;
    let mask = 2_i32.pow(position as u32);

    for num in input {
        num_1 += (num & mask) >> position;
    }

    let num_0 = input.len() as i32 - num_1;
    if num_0 > num_1 {
        0
    } else if num_1 > num_0 {
        1
    } else {
        -1
    }
}

fn get_value_with_criteria<T>(input: &Vec<i32>, get_criteria: T) 
    -> i32 
where 
    T: Fn(usize, &Vec<i32>) -> i32
{
    // All valid at the beginning
    let mut valid = (0..input.len() as i32).collect::<Vec<i32>>();
    // Get vector with numbers that correspond to valid indexes
    let mut masked_vec = input.clone();
    let mut position = 11;
    
    // Iterate till valid has a single number
    while valid.len() > 1 && position > 0 {
        let mut valid_indexes: Vec<i32> = Vec::new();

        // Calculate criteria for position
        let criteria= get_criteria(position, &masked_vec);

        // Calculate new valid numbers and save their indexes
        for (index, num) in masked_vec.iter().enumerate() {
            if ((num & 2_i32.pow(position as u32)) >> position) == criteria {
                valid_indexes.push(index as i32);
            }
        }

        valid = valid_indexes;
        // Get vector with numbers that correspond to valid indexes
        masked_vec = get_masked_vec(&masked_vec, &valid);
        position -= 1;
    }

    masked_vec.first().unwrap().to_owned()
}

fn get_masked_vec(input: &Vec<i32>, mask: &Vec<i32>) -> Vec<i32> {
    let mut masked_vec = Vec::new();

    for mask_index in mask.to_owned() {
        let val = input.get(mask_index as usize).unwrap().to_owned();
        masked_vec.push(val);
    }

    masked_vec
}

fn read_vec_input(input_path: &Path) -> Vec<i32> {
    let file = File::open(&input_path).ok().unwrap();
    let file_reader = BufReader::new(file);
    let mut vector: Vec<i32> = Vec::new();

    for line in file_reader.lines() {
        match line {
            Err(err) => panic!("encountered error: {:?}", err),
            Ok(line_string) => {
                let val = i32::from_str_radix(line_string.as_str(), 2).unwrap();
                vector.push(val);
            }
        }
    }

    vector
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_bin() {
        let input = vec![1, 2, 3];
        assert_eq!(1, most_common_bit(1, &input));
        assert_eq!(3, get_gamma(&input));
    }

    #[test]
    fn test_aoc() {
        let input = vec![
            0b000011000110,
            0b100110100101,
            0b101100101001,
            0b001100010000,
            0b011000100100,
            0b110100101111,
            0b110110001001,
            0b010010100101,
            0b100111000010,
        ];
        let expected = vec![1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1];

        for i in 0..12 {
            assert_eq!(expected[i], most_common_bit(i, &input))
        }

        let gamma = get_gamma(&input);
        assert_eq!(0b100110100101, gamma);
        assert_eq!(0b011001011010, gamma ^ 0b111111111111);
    }

    #[test]
    fn test_masked_vec() {
        let mut input = vec![0, 1, 2, 3, 4];
        let mut mask = vec![0, 3, 4];
        assert_eq!(vec![0, 3, 4], get_masked_vec(&input, &mask));

        input = vec![0, 1, 2, 3, 12, 123, 122];
        mask = (0..input.len() as i32).collect::<Vec<i32>>();
        assert_eq!(input, get_masked_vec(&input, &mask));
    }

    #[test]
    fn test_get_value() {
        let input = vec![
            0b000011000110,
            0b100110100101,
            0b101100101001,
            0b001100010000,
            0b011000100100,
            0b110100101111,
        ];

        let found = get_value_with_criteria(&input, |position, input| {
            match most_common_bit(position, input) {
                1 => 1,
                _ => 0
            } 
        });

        let expected = 0b000011000110;

        assert_eq!(expected, found);
    }
}