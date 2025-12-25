use std::{collections::{HashMap}, fs, };
// c1 and c2 represent an index into coords
#[derive(Debug)]
struct Distance{
    c1: usize, 
    c2: usize,
    distance: i64,
}

fn find(parents: &mut Vec<usize>, idx: usize) -> usize {
    if parents[idx] == idx {
        return idx
    }
    let root = find(parents, parents[idx]);
    parents[idx] = root;
    return root;
}

fn union(parents: &mut Vec<usize>, a:usize, b:usize){

    let p1 = find(parents, a);
    let p2 = find(parents, b);
    parents[p1] = p2
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    const ITERATIONS: usize = 1000;


    let coords: Vec<Vec<i64>> = file
        .lines()
        .map(|line| line.split(',').map(|s| s.parse::<i64>().unwrap()).collect())
        .collect();

    let n_coords = coords.len();
    let mut pairs: Vec<Distance>  =  Vec::new();
    
    for i in 0..n_coords{
        for j in i+1..n_coords{
            let distance: i64 = coords[i].iter().zip(coords[j].iter()).map(|(x,y)| (x-y).pow(2)).sum();
            pairs.push(Distance { c1: i, c2: j, distance });
        }
    }

    pairs.sort_by_key(|f| f.distance);

    let mut union_find: Vec<usize> = (0..n_coords).collect(); 

    
    for i in 0..ITERATIONS{
        let Distance {c1, c2, ..} = pairs[i];
        union(&mut union_find, c1, c2 );
    }

    // compress all paths
    for i in 0..n_coords{
        find(&mut union_find, i );
    }

    let mut counts:HashMap<usize, i32> = HashMap::new();

    for i in union_find.iter(){
        *counts.entry(*i).or_insert(0)+=1;
    }

    // get product of top three values
    let mut vals: Vec<i32> = counts.values().copied().collect();
    vals.sort_by(|a, b|b.cmp(a));
    vals.truncate(3);
    let answer:i32 = vals.iter().product();

    println!("{answer}");

}
    