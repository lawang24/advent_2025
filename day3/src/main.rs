use std::fs::File;
use std::io::{self,BufRead, BufReader};
use std::cmp::max;
use std::env;

fn main() -> io::Result<()>{

    let file_name = env::args().nth(1).unwrap();
    let path = format!("{}.txt", file_name);
    let file = File::open(path)?;
    
    let reader = BufReader::new(file);

    let mut answer = 0; 

    for line in reader.lines(){
        answer += get_number(line?);
    }

    println!("Answer: {answer}");

    Ok(())
}

fn get_number(x: String) -> usize {

    let mut answer = [0usize;12];
    let digits = x.chars().count() as i32;

    for (i, c) in x.chars().enumerate(){

        let curr= c.to_digit(10).unwrap() as usize;

        let digits_left  = digits - (i as i32); 

        let mut start = max(0, 12-digits_left) as usize;

        while start < 12{
            if curr > answer[start]{
                answer[start] = curr as usize;
                for i in start+1 .. answer.len(){
                    answer[i] = 0;
                }
                break;
            }
            else {
                start+=1;
            }
        }

    }
    let answer = answer.iter().fold(0usize,|acc,&d| acc * 10 + d);

    println!("{answer}");
    return answer;

}
