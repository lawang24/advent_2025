use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let first_line = input.lines().next().unwrap();
    let mut lasers = vec![false; first_line.len()];

    let mut answer = 0;

    for line in input.lines() {
        let mut next_lasers = lasers.clone();

        for (idx, c) in line.chars().enumerate() {

            if c == 'S'{
                next_lasers[idx] = true;
            }
            if c == '^' && lasers[idx]{
                if idx > 0 {
                    next_lasers[idx - 1] = true;
                }
                if idx < next_lasers.len() - 1 {
                    next_lasers[idx + 1] = true;
                }
                next_lasers[idx] = false;
                answer += 1;
            }
        }
        lasers = next_lasers;
    }
    println!("{answer}");
}
