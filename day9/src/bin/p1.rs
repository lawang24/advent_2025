use std::fs;

struct Coord {
    row: i64,
    col: i64
}

fn main(){

    let contents = fs::read_to_string("input.txt").unwrap();

    let mut coords: Vec<Coord> = Vec::new();

    for line in contents.lines(){
        let (row, col) = line.split_once(',').unwrap();
        coords.push(Coord { row: row.parse().unwrap(), col: col.parse().unwrap() });
    }

    let ncoords = coords.len();
    let mut rectangles: Vec<i64> = Vec::new(); 

    for i in 0..ncoords {
        for j in i+1..ncoords {
            rectangles.push(((coords[i].row-coords[j].row).abs()+1) * ((coords[i].col-coords[j].col).abs()+1));
        }
    }

    println!("{}", rectangles.iter().max().unwrap());

}