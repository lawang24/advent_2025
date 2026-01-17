use std::fs;

fn solve(puzzle: &str, piece_counts: &Vec<usize>) -> bool {
    let (board, counts) = puzzle.split_once(':').unwrap();
    let counts: Vec<&str> = counts.split_ascii_whitespace().collect();
    let (nrows, ncols) = board.split_once('x').unwrap();
    let nrows: usize = nrows.parse().unwrap();
    let ncols: usize = ncols.parse().unwrap();

    let total_pieces: usize = piece_counts
        .iter()
        .enumerate()
        .map(|(i, npiece)| npiece * (counts[i].parse::<usize>().unwrap()))
        .sum();

    return total_pieces <= nrows * ncols;
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut sections = contents.split("\n\n");
    let mut pieces: Vec<Vec<&str>> = Vec::new();
    let mut counts = Vec::new();

    // process the buttons
    for _ in 0..=5 {
        let button = &sections.next().unwrap()[3..];
        let key_count = button.matches('#').count();
        let button: Vec<&str> = button.split('\n').collect();
        pieces.push(button);
        counts.push(key_count);
    }

    let puzzles: Vec<&str> = sections.next().unwrap().split('\n').collect();
    let mut answer = 0;

    for puzzle in puzzles {
        if solve(puzzle, &counts) {
            answer += 1;
        }
    }

    dbg!(answer);
}
