use std::fs;

struct Range {
    start: u64,
    end: u64,
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
    let contents = fs::read_to_string("input.txt").unwrap();
    let parts: Vec<&str> = contents.trim().split("\n\n").collect();

    let range_strings: Vec<&str> = parts[0].lines().collect();
    let inputs: Vec<u64> = parts[1]
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let mut ranges: Vec<Range> = Vec::new();

    // preprocess ranges into bounds
    for curr in range_strings {
        ranges.push(Range::from_str(curr))
    }

    let mut answer = 0;

    for i in inputs{
        for &Range {start , end} in &ranges{
            if i >= start && i <= end {
                answer+=1;
                break
            }
        }
    }

    println!("{answer}");

}
