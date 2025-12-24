use std::fs;
use std::cmp;
use std::env;

struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn from_str(s: &str) -> Range {
        let (start, end) = s.split_once('-').unwrap();
        Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }
}

fn main() {

    let file = env::args().nth(1).unwrap_or("input".to_string())+".txt";
    let contents = fs::read_to_string(file).unwrap();
    let parts: Vec<&str> = contents.trim().split("\n\n").collect();

    let range_strings: Vec<&str> = parts[0].lines().collect();
    let mut ranges: Vec<Range> = Vec::new();

    // preprocess strings reps
    for curr in range_strings {
        ranges.push(Range::from_str(curr))
    }

    ranges.sort_by_key(|r| (r.end, r.start));

    let mut answer = 0;

    let mut left = 0;
    let mut right = -1;

    for &Range {start , end} in &ranges{

        // new interval
        if start > right{
            answer += right - left + 1;
            left = start;
            right = end;
        }
        else // merge interval
        {
            left = cmp::min(left, start);
            right = cmp::max(right, end);
        }
    }

    answer += right - left + 1;

    println!("{answer}");

}
