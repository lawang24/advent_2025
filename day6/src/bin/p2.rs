use std::fs;

enum CurrentOperation {
    Multiply,
    Add,
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();

    let nrows = grid.len();
    let ncols = grid[0].len();

    // iterate through the columns,
    // if the last row hits "+" or "*", we reset a new sequence
    // reading upwards through the row

    let mut curr_op = CurrentOperation::Add;
    let mut base: u64 = 0;
    let mut answer = 0;

    for ccol in 0..ncols {
        let operator = grid[nrows - 1][ccol];
        curr_op = match operator {
            '*' => {
                answer += base;
                base = 1;
                CurrentOperation::Multiply
            }
            '+' => {
                answer += base;
                base = 0;
                CurrentOperation::Add
            }
            _ => curr_op,
        };

        let mut total = 0u64;
        for crow in 0..nrows - 1 {
            let new= grid[crow][ccol].to_digit(10).unwrap_or(0) as u64;
            if new != 0{
                total = total * 10 + new;
            }
        }

        match curr_op {
            CurrentOperation::Multiply => {
                if total > 0{
                    base *= total;
                }
            }
            CurrentOperation::Add => {
                base += total;
            }
        }
    }

    answer += base;
    
    println!("{answer}");
}
