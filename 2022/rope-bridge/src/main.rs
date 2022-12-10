use std::io::{self, Read, Write};
use std::collections::HashSet;
use std::ops;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut head = Coordinates::origin();
    let mut tail = head.clone();

    let moves: Vec<(&str, i32)> = input.lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .map(|pairs| (pairs[0], pairs[1].parse().unwrap()))
        .collect();

    let mut seen: HashSet<Coordinates> = vec![Coordinates::origin()].into_iter().collect();
    for (dir, steps) in moves {
        head = head.move_op(dir, steps);
        if let Some(visited) = tail.follow(&head) {
            tail = visited.last().unwrap().clone();
            for visit in visited {
                seen.insert(visit);
            }
        }
    }

    writeln!(io::stdout(), "{}", seen.len())?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut head = Coordinates::origin();
    let mut tail = head.clone();

    let moves: Vec<(&str, i32)> = input.lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .map(|pairs| (pairs[0], pairs[1].parse().unwrap()))
        .collect();

    let mut history: Vec<Vec<Coordinates>> = vec![vec![Coordinates::origin()]; 10];
    for (dir, steps) in moves {
        head = head.move_op(dir, steps);
        if let Some(visited) = tail.follow(&head) {
            tail = visited.last().unwrap().clone();
            for visit in visited {
                history[0].push(visit);
            }
        }
    }

    // Each rope link has to follow the previous rope link
    for i in 1..9 {
        tail = Coordinates::origin();
        for coor in history[i-1].clone() {
            if let Some(visited) = tail.follow(&coor) {
                tail = visited.last().unwrap().clone();
                for visit in visited {
                    history[i].push(visit);
                }
            }
        }
    }

    writeln!(io::stdout(), "{}", history[8].clone().into_iter().collect::<HashSet<_>>().len())?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i32,
    y: i32
}

impl ops::Add<(i32,i32)> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: (i32,i32)) -> Self::Output {
        Coordinates { x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

impl ops::Add<(i32,i32)> for &Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: (i32,i32)) -> Self::Output {
        Coordinates { x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

impl Coordinates {
    fn origin() -> Self {
        Coordinates { x: 0, y: 0 }
    }

    fn manhattan_distance(&self, coordinate: &Coordinates) -> (i32, i32) {
        (i32::abs(self.x - coordinate.x), i32::abs(self.y - coordinate.y))
    }

    fn is_adjacent(&self, coordinate: &Coordinates) -> bool {
        let (x, y) = self.manhattan_distance(coordinate);

        x <= 1 && y <= 1
    }

    // Recursively gets closer to passed coordinate
    // At each iteration you get a step closer to the specified coordinate
    fn follow(&self, coordinate: &Coordinates) -> Option<Vec<Coordinates>> {
        if self.is_adjacent(coordinate) {
            return None
        }

        let mv = match (coordinate.x == self.x, coordinate.y == self.y) {
            (true, false) => {
                match self.y < coordinate.y {
                    true => (0,1),
                    false => (0,-1)
                }
            }
            (false, true) => {
                match self.x < coordinate.x {
                    true => (1,0),
                    false => (-1,0)
                }
            }
            (false, false) => {
                match (self.x < coordinate.x, self.y < coordinate.y) {
                    (true, true) => (1,1),
                    (true, false) => (1,-1),
                    (false, true) => (-1,1),
                    (false, false) => (-1,-1),
                }
            }
            _ => panic!("this is not reachable"),
        };

        let mut passed: Vec<Coordinates> = vec![(self + mv)];
        if let Some(coords) = passed.last()?.follow(coordinate) {
            for coord in coords {
                passed.push(coord);
            }
        }

        Some(passed)
    }

    // Return new coordinates after n steps in a particular direction
    fn move_op(self, dir: &str, steps: i32) -> Coordinates {
        let op = match dir {
            "R" => (steps,0),
            "L" => (-steps,0),
            "U" => (0,steps),
            "D" => (0,-steps),
            _ => panic!()
        };

        self + op
    }
}