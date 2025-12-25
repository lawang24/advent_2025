use std:: fs;
// c1 and c2 represent an index into coords
#[derive(Debug)]
struct Distance{
    c1: usize, 
    c2: usize,
    distance: i64,
}
struct UnionFindStruct{
    parents : Vec<usize>,
    sizes: Vec<usize>,
}

fn find(parents: &mut Vec<usize>, idx: usize) -> usize {
    if parents[idx] == idx {
        return idx
    }
    let root = find(parents, parents[idx]);
    parents[idx] = root;
    return root;
}

fn union(union_find: &mut UnionFindStruct, a:usize, b:usize){
    let p1 = find(&mut union_find.parents, a);
    let p2 = find(&mut union_find.parents, b);

    if p1 == p2 {
        return;
    }

    let UnionFindStruct {parents, sizes} = union_find;

    if sizes[p1] > sizes[p2]{
        parents[p2] = p1;
        sizes[p1] += sizes[p2];
    }
    else {
        parents[p1] = p2;
        sizes[p2] += sizes[p1];
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

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

    let mut union_find = UnionFindStruct{
        parents: (0..n_coords).collect(),
        sizes : vec![1;n_coords],
    };
    
    for i in 0..pairs.len(){
        let Distance {c1, c2, ..} = pairs[i];
        let p1 = find(&mut union_find.parents, c1);
        let p2 = find(&mut union_find.parents, c2);
        if p1 == p2 {
            continue
        }
        if union_find.sizes[p1] + union_find.sizes[p2] == n_coords {
            println!("Indices of last two connected circuits: {c1} {c2}");
            println!("X Coordinates: {} {}", coords[c1][0],coords[c2][0]);
            println!("Multiplied coordinates (answer): {}", coords[c1][0] * coords[c2][0]);
            break;
        }
        union(&mut union_find, c1, c2 );
    }

}
    