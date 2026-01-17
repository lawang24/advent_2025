use std::fs;

fn solve(puzzle: &str, pieces: &Vec<Vec<&str>>) -> u128 {
    
    let (board, counts) = puzzle.split_once(':').unwrap();
    let (nrows, ncols) = board.split_once('x').unwrap();
    let counts: Vec<&str> = counts.split_whitespace().collect();

    dbg!(nrows, ncols);
    dbg!(counts);

    0

}

fn main(){

    let contents = fs::read_to_string("test.txt").unwrap();
    let mut sections = contents.split("\n\n");
    let mut pieces: Vec<Vec<&str>> = Vec::new();

    // process the buttons
    for _ in 0..=5{
        let button = &sections.next().unwrap()[3..];
        let button: Vec<&str> = button.split('\n').collect();
        pieces.push(button);
    }

    let puzzles: Vec<&str> = sections.next().unwrap().split('\n').collect();

    for puzzle in puzzles{
        _ = solve(puzzle, &pieces);
    }
    
}