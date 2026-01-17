use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let mut answer: i32 = 0;
    let mut curr = 50;


    for line in reader.lines() {
        let line: String = line.unwrap();
        let sign: char = line.chars().next().unwrap();
        let number: i32 = line[1..].parse().unwrap();

        match sign {
            'L' => curr -= number,
            'R' => curr += number,
            _ => {}
        }

        curr %= 100;

        if curr == 0{
            answer +=1;
        }

    }
    println!("{answer}");
}
