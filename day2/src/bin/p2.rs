use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn is_magic_number(input: &str) -> bool {

    let n = input.len();

    for section_length in 1..(n/2+1){

        if n % section_length != 0{
            continue;
        }
        
        let mut pointer = section_length;
        let mut live = true;

        while pointer < n{
            if input[pointer-section_length..pointer] != input[pointer..pointer+section_length]{
                live = false;
                break
            }
            pointer+=section_length;
        }

        if live {
            return true;
        }
    }   

    false

}

fn main() -> io::Result<()> {
    // iterate through the numbers 

    // see if it's a magic number

    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let inputs = line.trim().split(',');

    let mut answer = 0;

    for line in inputs {
        let (a, b) = line
            .split_once('-').unwrap();
        answer += calculate(a, b);
    }

    println!("{answer}");

    Ok(())
}

fn calculate(a:&str, b:&str) -> u64 {

    let mut answer: u64 = 0;

    let lower = a.parse::<u64>().unwrap();
    let upper = b.parse::<u64>().unwrap();

    for curr in lower..upper+1{
        if is_magic_number(&curr.to_string()){
            answer+=curr;
        }
    }
    answer
}
