use std::{io::{Read, Write}, vec, collections::HashSet};


type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut xy = vec![0, 0];

    let mut curr: usize = 0;
    let dir_ops = vec![1,0,1,0];
    let move_ops = vec![1,1,-1,-1];

    for move_rep in input.replace(",", "").split(' ') {
        let (dir, steps) = (&move_rep[..1], &move_rep[1..].parse::<i32>()?);

        match dir {
            "R" => curr += 1,
            "L" => curr -= 1,
            _ => {}
        };

        xy[dir_ops[curr % 4]] += (move_ops[curr % 4]) * steps;
    }

    writeln!(std::io::stdout(), "{}", xy[0].abs() + xy[1].abs())?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut points: HashSet<(i32, i32)> = HashSet::new();
    points.insert((0, 0));

    let mut xy = vec![0,0];

    let mut curr: usize = 0;
    let dir_ops = vec![1,0,1,0];
    let move_ops = vec![1,1,-1,-1];

    for move_rep in input.replace(",", "").split(' ') {
        let (dir, steps) = (&move_rep[..1], &move_rep[1..].parse::<i32>()?);

        match dir {
            "R" => curr += 1,
            "L" => curr -= 1,
            _ => {}
        };

        for _ in 1..steps+1 {
            xy[dir_ops[curr % 4]] += (move_ops[curr % 4]) * 1;

            if points.contains(&(xy[0], xy[1])) {
                writeln!(std::io::stdout(), "{}", xy[0].abs() + xy[1].abs())?;
                return Ok(());
            } else {
                points.insert((xy[0], xy[1]));
            }
        }
    } 

    Ok(())
}