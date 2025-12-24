use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let first_line = input.lines().next().unwrap();
    let mut lasers = vec![0u64; first_line.len()];

    for line in input.lines() {
        let mut next_lasers = lasers.clone();

        for (idx, c) in line.chars().enumerate() {

            if c == 'S'{
                next_lasers[idx] = 1;
            }
            if c == '^' && lasers[idx] > 0 {
                if idx > 0 {
                    next_lasers[idx - 1] += lasers[idx];
                }
                if idx < next_lasers.len() - 1 {
                    next_lasers[idx + 1] += lasers[idx];
                }
                next_lasers[idx] -= lasers [idx];
            }
        }
        lasers = next_lasers;
    }

    let answer: u64 = lasers.iter().sum();
    println!("{answer}");

}
