use std::{
    fs::File, 
    io::{
        BufReader,
        BufRead
    }, 
    path::Path, 
    collections::HashMap, 
    hash::Hash
};

fn main() {
    let input = read_input(Path::new("input"));
    let grouped = input.group();
    let max = get_max(&grouped);

    // Part one
    let mut fuel;
    let mut fuel_vec = Vec::new();
    for i in 0..max {
        fuel = 0;
        for (key, _) in &grouped {
            fuel += (i - *key).abs() * grouped.get(&key).unwrap();
        }
        fuel_vec.push(fuel);
    }

    let mut min_fuel = fuel_vec.iter().min().unwrap();
    println!("{}", min_fuel);

    // Part two
    fuel_vec = Vec::new();
    for i in 0..max {
        fuel = 0;
        for (key, _) in &grouped {
            fuel += get_distance_triangular_num(i, *key) * grouped.get(&key).unwrap();
        }
        fuel_vec.push(fuel);
    }
    min_fuel = fuel_vec.iter().min().unwrap();
    println!("{}", min_fuel);
}

// https://www.mathsisfun.com/algebra/triangular-numbers.html
fn get_distance_triangular_num(a: i32, b: i32) -> i32 {
    let distance = (a-b).abs();

    distance * (distance + 1) / 2
}

fn get_max(map: &HashMap<i32, i32>) -> i32 {
    *map.keys().into_iter().max().unwrap()
}

trait Group<T> {
    fn group(self) -> HashMap<T, i32>;
}

impl<T> Group<T> for Vec<T> where T: Hash + Eq {
    fn group(self) -> HashMap<T, i32> {
        let mut hash = HashMap::new();

        for num in self {
            if hash.contains_key(&num) {
                *hash.get_mut(&num).unwrap() += 1;
            } else {
                hash.insert(num, 1);
            }
        }

        hash
    }
}

fn read_input(path: &Path) -> Vec<i32> {
    let file = File::open(path).ok().unwrap();
    let mut file_reader = BufReader::new(file);

    let mut buffer = String::new();
    file_reader.read_line(&mut buffer).unwrap();

    let mut nums = Vec::new();
    for num in buffer.split(",").into_iter() {
        let num = num.trim().parse().ok().unwrap();
        nums.push(num);
    }

    nums
}