use std::fs;
use std::cmp::max;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<_> = contents.split('\n').collect();
    let mut answer = 0;

    for line in lines {
        answer += get_number(line);
    }

    println!("Answer: {answer}");
}

fn get_number(x: &str) -> usize {

    let mut answer = [0usize;2];
    let digits = x.chars().count() as i32;

    for (i, c) in x.chars().enumerate(){

        let curr= c.to_digit(10).unwrap() as usize;

        let digits_left  = digits - (i as i32); 

        let mut start = max(0, 2-digits_left) as usize;

        while start < 2{
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

    return answer;

}
