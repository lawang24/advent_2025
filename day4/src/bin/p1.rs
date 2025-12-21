use std::fs::File;
use std::io::{BufRead, BufReader};

const DIRS: [i32; 3] = [-1, 0, 1];

fn is_in_bounds(row: i32, col: i32, nrows: i32, ncols: i32) -> bool {
    return row >= 0 && row < nrows && col >= 0 && col < ncols;
}

fn count_nearby_rolls(row: usize, col: usize, grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let nrows = grid.len() as i32;
    let ncols = grid[0].len() as i32;

    for dr in DIRS {
        for dc in DIRS {
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            if !(dr == 0 && dc == 0)
                && is_in_bounds(nr, nc, nrows, ncols)
                && grid[nr as usize][nc as usize] == '@'
            {
                count += 1;
            }
        }
    }

    return count

}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        grid.push(line.chars().collect())
    }

    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut answer = 0;

    for row in 0..nrows {
        for col in 0..ncols {
            if grid[row][col] == '@' && count_nearby_rolls(row, col, &grid) < 4 {
                answer +=1 ;
            }
        }
    }
    println!("{answer}");
}
