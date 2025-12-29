use std::{collections::HashMap, fs, hash::Hash, iter::Enumerate};

struct Coord {
    row: i64,
    col: i64,
}

fn main() {
    let contents = fs::read_to_string("test.txt").unwrap();

    let mut x_coords: Vec<i64> = Vec::new();
    let mut y_coords: Vec<i64> = Vec::new();

    for line in contents.lines() {
        let (row, col) = line.split_once(',').unwrap();
        x_coords.push(row.parse().unwrap());
        y_coords.push(col.parse().unwrap());
    }

    let ncoords = x_coords.len();

    // coordinate compression algorithm
    x_coords.sort_unstable();
    y_coords.sort_unstable();
    x_coords.dedup();
    y_coords.dedup();
    let mut x_map: HashMap<i64, usize> = HashMap::new();
    let mut y_map: HashMap<i64, usize> = HashMap::new();
    for (i, v) in x_coords.iter().enumerate(){
        x_map.insert(*v, i);
    }
    for (i, v) in y_coords.iter().enumerate(){
        y_map.insert(*v, i);
    }

}
