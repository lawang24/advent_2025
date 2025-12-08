use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> io::Result<()> {
    // iterate through the numbers that need to be "doubled"

    // For the lower bound need to find the smallest number that is greater
    // case even digits: take the first half of numbers. If first_half < second_half, first_half+=1. This is smallest double
    // that is guaranteed greater than the lower bound.
    // case odd digits: the next "doubled" number will guaranteed be larger. smallest double is 100...100....
    // if x digits, first_half = 10 ^ ((x+1)/2)

    // For the upper bound, find the largest number that is smaller than A
    // case even digits: take the first half of numbers. If first_half > second_half, first_half-=1. This is largest double
    // that is guaranteed greater than the upper bound.
    // case odd digits: the "doubled" number will x-1 digits long . smallest double is 999999
    // if x digits, first_half = 9999.. (x-1) times

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line?;
        let (a, b) = line
            .split_once('-').unwrap();
    }

    Ok(())
}

fn calculate(a:String, b:String) -> i32 {

    let a_len = a.len();
    let mut lower = 0;
    
    // lower bound even
    if a_len % 2 == 0 {
        let (first, second) = a.split_at(a_len/2);
        let n1 = first.parse::<i32>().unwrap();
        let n2 = second.parse::<i32>().unwrap();
        lower = if n1 >= n2 {n1} else {n1+1};
    }
    else {
        lower = 10_i32.pow(((a_len+1)/2) as u32);
    }

    let b_len = b.len();
    let mut upper:i32 = 0;

    if b_len%2 == 0 {
        let (first, second) = b.split_at(b_len/2);
        let n1 = first.parse::<i32>().unwrap();
        let n2 = second.parse::<i32>().unwrap();
        upper = if n1 >= n2 {n1} else {n1+1};
    }

    



    return 0;
}
