use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut curr: i32 = 50;
    let mut answer: i32 = 0;

    for line in reader.lines() {
        let line: String = line?;
        let sign: char = line.chars().next().unwrap();
        let number: i32 = line[1..].parse().unwrap();

        let start_from_zero = curr == 0;

        match sign {
            'L' => curr -= number,
            'R' => curr += number,
            _ => {}
        }

        // you can't start from 100, since we mod 100
        // add how many times you went over
        if curr >= 100 {
            answer += curr / 100;
            curr %= 100;
        }
        // you can start from 0, only add if you crossed over 
        else if curr <= 0 {
            // extra rotation from crossing over zero
            if !start_from_zero {
                answer += 1;
            }

            answer += -1 * curr / 100;
            curr = (curr % 100 + 100) % 100;
        }

    }

    println!("{}", answer);

    Ok(())
}
