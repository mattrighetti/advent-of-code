use std::io::{self, Read, Write};
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let grid = SquareGrid::from_str(input)?;

    writeln!(io::stdout(), "{}", grid.get_total_visible())?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let grid = SquareGrid::from_str(input)?;
    let mut score = 0;

    for i in 0..grid.size {
        for j in 0..grid.size {
            score = std::cmp::max(grid.get_scenic_score((i, j)), score);
        }
    }

    writeln!(io::stdout(), "{}", score)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct SquareGrid {
    size: usize,
    matrix: Vec<Vec<i32>>
}

impl SquareGrid {
    fn get_external_visible(&self) -> i32 {
        (self.matrix.len() as i32 * 4) - 4
    }

    fn get_inner_visible_mask_from_right(&self) -> Vec<Vec<bool>> {
        let mut mask: Vec<Vec<bool>> = vec![vec![false; self.size]; self.size];
        for i in 1..(self.size - 1) {
            let mut prev = self.matrix[i][self.size - 1];
            for j in (1..(self.size - 1)).rev() {
                if self.matrix[i][j] > prev {
                    prev = self.matrix[i][j];
                    mask[i][j] = true;
                }
            }
        }

        mask
    }

    fn get_inner_visible_mask_from_left(&self) -> Vec<Vec<bool>> {
        let mut mask: Vec<Vec<bool>> = vec![vec![false; self.size]; self.size];
        for i in 1..(self.size - 1) {
            let mut prev = self.matrix[i][0];
            for j in 1..(self.size - 1) {
                if self.matrix[i][j] > prev {
                    prev = self.matrix[i][j];
                    mask[i][j] = true;
                }
            }
        }

        mask
    }

    fn get_inner_visible_mask_from_top(&self) -> Vec<Vec<bool>> {
        let mut mask: Vec<Vec<bool>> = vec![vec![false; self.size]; self.size];
        for i in 1..(self.size - 1) {
            let mut prev = self.matrix[0][i];
            for j in 1..(self.size - 1) {
                if self.matrix[j][i] > prev {
                    prev = self.matrix[j][i];
                    mask[j][i] = true;
                }
            }
        }

        mask
    }

    fn get_inner_visible_mask_from_bottom(&self) -> Vec<Vec<bool>> {
        let mut mask: Vec<Vec<bool>> = vec![vec![false; self.size]; self.size];
        for i in 1..self.size - 1 {
            let mut prev = self.matrix[self.size - 1][i];
            for j in (0..(self.size - 1)).rev() {
                if self.matrix[j][i] > prev {
                    prev = self.matrix[j][i];
                    mask[j][i] = true;
                }
            }
        }

        mask
    }

    fn get_inner_visible(&self) -> i32 {
        let (r_mat, l_mat, t_mat, b_mat) = (
            self.get_inner_visible_mask_from_right(),
            self.get_inner_visible_mask_from_left(),
            self.get_inner_visible_mask_from_top(),
            self.get_inner_visible_mask_from_bottom()
        );

        let mut tot_visible = 0;
        for i in 0..r_mat.len() {
            for j in 0..r_mat.len() {
                if r_mat[i][j] || l_mat[i][j] || t_mat[i][j] || b_mat[i][j] {
                    tot_visible += 1;
                }
            }
        }

        tot_visible
    }

    fn get_total_visible(&self) -> i32 {
        self.get_external_visible() + self.get_inner_visible()
    }

    fn get_scenic_score(&self, pos: (usize, usize)) -> i32 {
        let mut res = 1;
        let pos_height = self.matrix[pos.0][pos.1];

        let mut r_pos = pos.1;
        loop {
            if r_pos < self.size - 1 {
                r_pos += 1;
                if self.matrix[pos.0][r_pos] >= pos_height {
                    break;
                }
            } else {
                break;
            }
        }

        let mut l_pos = pos.1;
        loop {
            if l_pos > 0 {
                l_pos -= 1;
                if self.matrix[pos.0][l_pos] >= pos_height {
                    break;
                }
            } else {
                break;
            }
        }

        let mut t_pos = pos.0;
        loop {
            if t_pos > 0 {
                t_pos -= 1;
                if self.matrix[t_pos][pos.1] >= pos_height {
                    break;
                }
            } else {
                break;
            }
        }

        let mut b_pos = pos.0;
        loop {
            if b_pos < self.size - 1 {
                b_pos += 1;
                if self.matrix[b_pos][pos.1] >= pos_height {
                    break;
                }
            } else {
                break;
            }
        }

        for v in vec![(r_pos, pos.1), (pos.1, l_pos), (pos.0, t_pos), (b_pos, pos.0)].into_iter()
            .map(|x| i32::abs(x.0 as i32 - x.1 as i32))
            .filter(|x| *x != 0)
        {
            res *= v;
        }

        res as i32
    }
}

impl FromStr for SquareGrid {
    type Err = Box<dyn ::std::error::Error>;

    fn from_str(s: &str) -> Result<SquareGrid> {
        let matrix: Vec<Vec<i32>> = s.lines()
        .map(|line| {
            line.chars()
            .map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>()
        })
        .collect();

        Ok(SquareGrid { size: matrix.len(), matrix })
    }
}