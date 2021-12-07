use std::{path::Path, io::{BufReader, BufRead}, fs::File};

fn main() {
    let moves = read_vec_input(Path::new("input"));
    
    // Part One
    let (depth, horizontal) = get_depth_and_horiziontal_position(moves.clone());
    println!("(depth, horizontal) = {:?},\nresult: {}", (depth, horizontal), depth*horizontal);

    // Part two
    let (depth, horizontal, aim) = get_depth_horiziontal_aim_position(moves);
    println!("(depth, horizontal, aim) = {:?},\nresult: {}", (depth, horizontal, aim), depth*horizontal);
}

#[derive(Debug, Clone)]
enum Move {
    Forward(i32),
    Up(i32),
    Down(i32)
}

fn get_depth_and_horiziontal_position(moves: Vec<Move>) -> (i32, i32) {
    let mut depth = 0;
    let mut horizontal = 0;

    for move_ in moves {
        match move_ {
            Move::Forward(num) => horizontal += num,
            Move::Down(num) => depth += num,
            Move::Up(num) => depth -= num
        }
    }

    (depth, horizontal)
}

fn get_depth_horiziontal_aim_position(moves: Vec<Move>) -> (i32, i32, i32) {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for move_ in moves {
        match move_ {
            Move::Forward(num) => {
                horizontal += num;
                depth += aim * num;
            },
            Move::Down(num) => aim += num,
            Move::Up(num) => aim -= num
        }
    }

    (depth, horizontal, aim)
}

fn parse_move(command: &str) -> Move {
    let chunk = command.split(' ').collect::<Vec<&str>>();

    match chunk.get(0).unwrap().to_owned() {
        "forward" => Move::Forward(chunk.get(1).unwrap().parse::<i32>().unwrap()),
        "down" => Move::Down(chunk.get(1).unwrap().parse::<i32>().unwrap()),
        "up" => Move::Up(chunk.get(1).unwrap().parse::<i32>().unwrap()),
        _ => panic!("not a valid move")
    }
}

fn read_vec_input(input_path: &Path) -> Vec<Move> {
    let file = File::open(&input_path).ok().unwrap();
    let file_reader = BufReader::new(file);
    let mut vector: Vec<Move> = Vec::new();

    for line in file_reader.lines() {
        match line {
            Err(err) => panic!("encountered error: {:?}", err),
            Ok(line_string) => {
                vector.push(parse_move(line_string.as_str()));
            }
        }
    }

    vector
}