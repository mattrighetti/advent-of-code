use std::{
    path::Path, 
    fs::File, 
    io::{
        self, 
        BufRead,
        BufReader
    }, 
    collections::HashMap,
    io::{Error, ErrorKind}
};

type Table = HashMap<i32, (i32, i32)>;

#[derive(Debug, Clone)]
struct StackTable {
    table: Table,
    checkstack: HashMap<i32, i32>,
    checkstack_col: HashMap<i32, i32>,
    total_score: i32,
    winner_call: Option<i32>,
    lock: bool
}

impl StackTable {
    fn new(slice: &[i32]) -> Self {
        let mut checkstack = HashMap::new();
        for i in 0..=4 {
            checkstack.insert(i, 0);
        }

        let mut checkstack_col = HashMap::new();
        for i in 0..=4 {
            checkstack_col.insert(i, 0);
        }

        let mut total_score = 0;
        for num in slice {
            total_score += num;
        }

        let table = get_hashtable(&slice);
        // I don't know bingo, so I'm checking that there are no duplicates in a table
        assert_eq!(25, table.keys().len());

        StackTable {
            table,
            checkstack,
            checkstack_col,
            total_score,
            winner_call: None,
            lock: false
        }
    }

    fn check(&mut self, num: i32) -> Option<i32> {
        if !self.lock {
            return match self.table.get(&num) {
                Some((row, col)) => {
                    self.total_score -= num;
                    
                    if let Some(val) = self.checkstack.get_mut(row) {
                        *val = *val + 1;
                    }
    
                    if let Some(val) = self.checkstack_col.get_mut(col) {
                        *val = *val + 1;
                    }
    
                    match (self.checkstack.get(row), self.checkstack_col.get(col)) {
                        (Some(5), _) => {
                            self.lock = true;
                            self.winner_call = Some(num);
                            return Some(*row);
                        },
                        (_, Some(5)) => {
                            self.lock = true;
                            self.winner_call = Some(num);
                            return Some(*row);
                        }
                        _ => None
                    }
                }
                None => None
            }
        }

        None
    }

    fn get_score(&self) -> Option<i32> {
        if let Some(winner_call) = self.winner_call {
            Some(self.total_score * winner_call)
        } else {
            None
        }
    }
}

fn main() {
    let (vec, mut tables) = read_vec_input(Path::new("input"));
    let mut table2 = tables.clone();

    // Part one
    let winner = get_winner_table(&vec, &mut tables).ok().unwrap();
    println!(
        "winner table: {:?}\nresult: {}", 
        winner,
        tables.get(winner as usize).unwrap().get_score().unwrap()
    );

    // Part two
    let last_winner = get_last_winning_table(&vec, &mut table2);
    println!(
        "last winner table: {:?}\nresult: {}", 
        last_winner,
        table2.get(last_winner as usize).unwrap().get_score().unwrap()
    );
}

fn get_winner_table(draw: &Vec<i32>, tables: &mut Vec<StackTable>) -> io::Result<i32> {
    for num in draw {
        for (index, table) in tables.iter_mut().enumerate() {
            if table.check(*num).is_some() {
                return Ok(index as i32);
            }
        }
    }

    Err(Error::new(ErrorKind::NotFound, "Could not find winning table"))
}

fn get_last_winning_table(draw: &Vec<i32>, tables: &mut Vec<StackTable>) -> i32 {
    let mut stack = Vec::new();
    for num in draw {
        for (index, table) in tables.iter_mut().enumerate() {
            if table.check(*num).is_some() {
                stack.push(index as i32);
            }
        }
    }

    stack.pop().unwrap()
}

fn read_vec_input(input_path: &Path) -> (Vec<i32>, Vec<StackTable>) {
    let file = File::open(input_path).ok().unwrap();
    let mut file_reader = BufReader::new(file);
    
    let mut buffer = String::new();
    file_reader.read_line(&mut buffer).unwrap();

    let mut draw = Vec::new();
    let mut cursor = io::Cursor::new(&buffer);
    for val in cursor
    .split(b',')
    .map(|l| 
        String::from_utf8(l.unwrap()).unwrap()
    ) {
        if let Some(val) = val.trim().parse::<i32>().ok() {
            draw.push(val);
        }
    }

    let mut all_vec = Vec::new();

    for line in file_reader.lines() {
        match line {
            Ok(ref string) => {
                cursor = io::Cursor::new(string);
                for val in cursor.split(b' ')
                .map(|l| 
                    String::from_utf8(l.unwrap()).unwrap()
                ) {
                    if let Some(val) = val.trim().parse::<i32>().ok() {
                        all_vec.push(val);
                    }
                }
            }
            Err(_) => panic!("cannot read line")
        }
    }

    let mut tables: Vec<StackTable> = Vec::new();

    let dst: Vec<&[i32]> = all_vec.chunks_exact(25).collect();
    for chunk_array in dst {
        tables.push(StackTable::new(chunk_array));
    }

    (draw, tables)
}

fn get_hashtable(slice: &[i32]) -> Table {
    let mut map: Table = HashMap::new();
    let mut row_selector = 0;
    let mut col_selector = 0;

    for num in slice {
        map.insert(*num, (col_selector, row_selector));
        row_selector = (row_selector + 1) % 5;
        if row_selector == 0 {
            col_selector = (col_selector + 1) % 5;
        }
    }

    map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stacktable() {
        let slice = [
            4,    12,   0,     1,   32, 
            48,   29,   299,   129, 10, 
            128,  1001, 12999, 193, 1023, 
            3238, 329,  388,   317, 3012, 
            438,  89,   760,   699, 79
        ];
        let mut stacktable = StackTable::new(&slice);
        assert_eq!(stacktable.total_score, 25257);

        let winning_set = [38, 29, 1, 0, 32, 12, 4, 48];
        let score = 25179;
        for num in winning_set {
            if let Some(_) = stacktable.check(num) {
                assert_eq!(stacktable.winner_call, Some(4));
                assert_eq!(stacktable.total_score, score);
                assert_eq!(stacktable.get_score().unwrap(), score * 4);
            }
        }
    }
}