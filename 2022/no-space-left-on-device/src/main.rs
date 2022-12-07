use std::{io::{self, Read, Write}, cmp};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut tot = 0;
    iterate_on_folders(input, |folder_size| {
        if folder_size < 100000 {
            tot += folder_size;
        }
    })?;

    writeln!(io::stdout(), "{}", tot)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let tot_used = get_root_size(input);

    let mut folder_to_delete_size = i32::MAX;
    iterate_on_folders(input, |folder_size| {
        if 70000000 - tot_used + folder_size >= 30000000 {
            folder_to_delete_size = cmp::min(folder_size, folder_to_delete_size);
        }
    })?;

    writeln!(io::stdout(), "{}", folder_to_delete_size)?;
    Ok(())
}

// Walks in folders as input instructs, a stack is used to keep track of the current folder
// When `$ cd ..` is read, you pop the current folder value in stack and you add that value
// to the outer folder, as that contains the folder just popped
fn iterate_on_folders<F>(input: &str, mut on_pop: F) -> Result<()> where F: FnMut(i32) {
    let mut folders_stack: Vec<i32> = Vec::new();

    for line in input.lines() {
        let pieces: Vec<&str> = line.split(" ").collect();
        match pieces.len() {
            // cd into new dir or go back to prev
            3 => {
                match pieces[2] {
                    ".." => {
                        if let Some(val) = folders_stack.pop() {
                            on_pop(val);
                            // Add this size to the outer folder, which is not last value in the stack
                            *folders_stack.last_mut().unwrap() += val;
                        }
                    },
                    _ => {
                        folders_stack.push(0);
                    }
                }
            },
            2 => {
                match pieces[0].parse::<i32>() {
                    Ok(val) => {
                        *folders_stack.last_mut().unwrap() += val;
                    }
                    // ignore `$ ls` and `dir .*`
                    Err(_) => continue,
                }
            },
            _ => Err("unrecognized")?
        };
    }

    Ok(())
}

// Skip all lines that are not of type `[0-9]+ [A-Za-z0-9\.]*`
// sum all the others to total used space
fn get_root_size(input: &str) -> i32 {
    let mut tot_used = 0;
    for line in input.lines() {
        let pieces: Vec<&str> = line.split(" ").collect();
        match (pieces.len(), pieces[0].parse::<i32>()) {
            (2, Ok(val)) => {
                tot_used += val;
            },
            (_, _) => continue
        }
    }

    tot_used
}