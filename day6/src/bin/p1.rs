use std::env;
use std::fs;

fn main() {
    // pass in text (test) or input (full)
    let file = env::args().nth(1).unwrap_or("input".to_string()) + ".txt";
    let contents = fs::read_to_string(file).unwrap();

    let grid: Vec<Vec<&str>> = contents
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let mut total = 0;
    let nrows = grid.len();
    let ncols = grid[0].len();

    for col in 0..ncols {

        let mut answer;
        if grid[nrows-1][col] == "+" {
            answer = 0;
            for crow in 0..nrows-1{
                answer += grid[crow][col].parse::<i64>().unwrap();
            }
        }
        else{
            answer = 1;
            for crow in 0..nrows-1{
                answer *= grid[crow][col].parse::<i64>().unwrap();
            }
        }

        total += answer;
    }

    println!("{total}")

}
