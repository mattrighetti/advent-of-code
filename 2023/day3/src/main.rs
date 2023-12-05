use core::panic;
use std::{
    collections::HashSet,
    io::{self, Read, Write},
    usize,
};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    writeln!(io::stdout(), "{}", part1(&input)?)?;
    writeln!(io::stdout(), "{}", part2(&input)?)?;
    Ok(())
}

fn part1(input: &str) -> io::Result<u32> {
    let mut mat: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        mat.push(line.chars().collect());
    }

    let mut sum = 0;
    for i in 0..mat.len() {
        let (mut l, mut r) = (0, 0);
        while l < mat[i].len() {
            // Position l to next digit char
            while l < mat[i].len() && !mat[i][l].is_digit(10) {
                l += 1;
            }

            if l == mat[i].len() {
                break;
            }

            r = l;
            // Position r to last digit char
            while r < mat[i].len() - 1 && mat[i][r + 1].is_digit(10) {
                r += 1;
            }

            // number is between r and l here
            let n_string: String = mat[i][l..=r].iter().collect();
            let n: u32 = n_string.parse().unwrap();

            while l <= r {
                // check neighboors
                if has_neighboring_symbol(&mat, i, l) {
                    sum += n;
                    l = r + 1;
                } else {
                    l += 1;
                }
            }
        }
    }

    Ok(sum)
}

fn part2(input: &str) -> io::Result<u32> {
    let mut mat: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        mat.push(line.chars().collect());
    }

    let mut ratios_num_coords = Vec::new();
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            if mat[i][j] == '*' {
                if let Some(coord) = neighboring_number_pair(&mat, i, j) {
                    ratios_num_coords.push(coord);
                }
            }
        }
    }

    let sum = ratios_num_coords
        .iter()
        .inspect(|pair| println!("{:?}", pair))
        .map(|(n1, n2)| n1 * n2)
        .sum();

    Ok(sum)
}

type Coord = (usize, usize);

fn neighboring_number_pair(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> Option<(u32, u32)> {
    let mut num = Vec::new();

    let (n, m) = (matrix.len() as i32, matrix[0].len() as i32);
    for (x, y) in get_neighbors_coords(n, m, i as i32, j as i32) {
        if matrix[x][y].is_digit(10) {
            num.push((x, y));
        }
    }

    let mut pairs: HashSet<u32> = HashSet::new();
    for coord in num {
        pairs.insert(get_num_at_coord(matrix, &coord));
    }
    let pairs_vec = Vec::from_iter(pairs.iter());

    match pairs_vec.len() {
        2 => Some((*pairs_vec[0], *pairs_vec[1])),
        _ => None,
    }
}

fn has_neighboring_symbol(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let (n, m) = (matrix.len() as i32, matrix[0].len() as i32);
    for (x, y) in get_neighbors_coords(n, m, i as i32, j as i32) {
        if is_symbol(&matrix[x][y]) {
            return true;
        }
    }

    false
}

fn get_neighbors_coords(n: i32, m: i32, i: i32, j: i32) -> HashSet<Coord> {
    let coords = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
    ];

    let mut nb = HashSet::new();
    for (x, y) in coords {
        nb.insert((i + x, j + y));
    }

    nb.into_iter()
        .filter(|(x, y)| !(*x < 0 || *y < 0 || *x >= n || *y >= m))
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

fn get_num_at_coord(matrix: &Vec<Vec<char>>, coord: &Coord) -> u32 {
    let row = coord.0;
    let (mut l, mut r) = (coord.1, coord.1);

    while l > 0 && matrix[row][l - 1].is_digit(10) {
        l -= 1;
    }

    while r < matrix[row].len() && matrix[row][r + 1].is_digit(10) {
        r += 1;
    }

    matrix[row][l..=r]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap()
}

fn is_symbol(c: &char) -> bool {
    c.is_ascii_punctuation() && *c != '.'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbors_coors() {
        assert_eq!(
            HashSet::from_iter(vec![(1, 0), (1, 1), (0, 1)]),
            get_neighbors_coords(7, 7, 0, 0)
        );
        assert_eq!(
            HashSet::from_iter(vec![(5, 6), (5, 5), (6, 5)]),
            get_neighbors_coords(7, 7, 6, 6)
        );
        assert_eq!(
            HashSet::from_iter(vec![
                (2, 2),
                (2, 3),
                (2, 4),
                (4, 2),
                (4, 3),
                (4, 4),
                (3, 2),
                (3, 4)
            ]),
            get_neighbors_coords(7, 7, 3, 3)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            4361,
            part1(
                r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )
            .unwrap()
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            467835,
            part2(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )
            .unwrap()
        );
        assert_eq!(
            (467 * 35) + (617 * 2) + (755 * 598),
            part2(
                "467..114..
...*......
..35..633.
......#...
617*2.....
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )
            .unwrap()
        );
    }
}
