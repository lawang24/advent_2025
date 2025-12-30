use std::{cmp, collections::HashMap, collections::VecDeque, fs};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum GridState {
    Border,
    Outside,
    Inside,
}

fn connect_coords(x1: usize, y1: usize, x2: usize, y2: usize, is_filled: &mut Vec<Vec<GridState>>) {
    // either x1 == x2 or y1 == y2 for adjacent coordinates based on problem constraints
    if x1 == x2 {
        for i in cmp::min(y1, y2)..cmp::max(y1, y2) {
            is_filled[x1][i] = GridState::Border;
        }
    } else {
        for i in cmp::min(x1, x2)..cmp::max(x1, x2) {
            is_filled[i][y1] = GridState::Border;
        }
    }
}

fn create_auxiliary_space(mut coord_sorted_set: Vec<i64>) -> Vec<i64> {
    let mut new_coords: Vec<i64> = vec![
        coord_sorted_set[0] - 1,
        coord_sorted_set[coord_sorted_set.len() - 1] + 1,
    ];

    for i in 1..coord_sorted_set.len() {
        if coord_sorted_set[i] > coord_sorted_set[i - 1] + 1 {
            new_coords.push(coord_sorted_set[i] - 1);
        };
    }

    coord_sorted_set.extend(&new_coords);
    coord_sorted_set.sort_unstable();
    coord_sorted_set.dedup();
    coord_sorted_set
}

fn flood_fill(mapped_x: usize, mapped_y: usize, is_filled: &mut Vec<Vec<GridState>>) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((mapped_x, mapped_y));

    while let Some((x, y)) = queue.pop_front() {
        if is_filled[x][y] != GridState::Inside {
            continue;
        }

        is_filled[x][y] = GridState::Outside;

        if x > 0{
            queue.push_back((x - 1, y));
        }
        if y > 0 {
            queue.push_back((x, y-1));
        }
        if x < is_filled.len() - 1 {
            queue.push_back((x+1, y));
        }
        if y < is_filled[0].len() - 1 {
            queue.push_back((x, y+1));
        }
    }
}

// x1 <= x2, y1 <= y2 for more convenient traversal
fn check_rectangle(
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    is_filled: &mut Vec<Vec<GridState>>,
) -> bool {
    for i in x1..=x2 {
        if is_filled[i][y1] == GridState::Outside || is_filled[i][y2] == GridState::Outside {
            return false;
        }
    }
    for i in y1..=y2 {
        if is_filled[x1][i] == GridState::Outside || is_filled[x2][i] == GridState::Outside {
            return false;
        }
    }
    return true;
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut x_coords: Vec<i64> = Vec::new();
    let mut y_coords: Vec<i64> = Vec::new();
    let mut coords: Vec<(i64, i64)> = Vec::new();

    for line in contents.lines() {
        let (row, col) = line.split_once(',').unwrap();
        let row: i64 = row.parse().unwrap();
        let col: i64 = col.parse().unwrap();
        x_coords.push(row);
        y_coords.push(col);
        coords.push((row, col));
    }

    // connect the polygon
    coords.push(coords[0]);

    // coordinate compression algorithm
    x_coords.sort_unstable();
    y_coords.sort_unstable();
    x_coords.dedup();
    y_coords.dedup();

    x_coords = create_auxiliary_space(x_coords);
    y_coords = create_auxiliary_space(y_coords);

    // create coordinate space in between
    let mut x_map: HashMap<i64, usize> = HashMap::new();
    let mut y_map: HashMap<i64, usize> = HashMap::new();
    for (i, v) in x_coords.iter().enumerate() {
        x_map.insert(*v, i);
    }
    for (i, v) in y_coords.iter().enumerate() {
        y_map.insert(*v, i);
    }

    let mut is_filled: Vec<Vec<GridState>> =
        vec![vec![GridState::Inside; y_map.len()]; x_map.len()];

    for i in 1..coords.len() {
        let ((x1, y1), (x2, y2)) = (coords[i - 1], coords[i]);
        connect_coords(
            *x_map.get(&x1).unwrap(),
            *y_map.get(&y1).unwrap(),
            *x_map.get(&x2).unwrap(),
            *y_map.get(&y2).unwrap(),
            &mut is_filled,
        );
    }
    // flood fill outside the polygon with GridState:Outside
    flood_fill(0, 0, &mut is_filled);

    let n_coords = coords.len();

    let mut champ = 0;

    // now check each rectangle's borders that there is no GridState:Outside
    for i in 0..n_coords {
        for j in i + 1..n_coords {
            let ((x1, y1), (x2, y2)) = (coords[i], coords[j]);
            let (x1, x2) = (cmp::min(x1, x2), cmp::max(x1, x2));
            let (y1, y2) = (cmp::min(y1, y2), cmp::max(y1, y2));

            if check_rectangle(
                *x_map.get(&x1).unwrap(),
                *y_map.get(&y1).unwrap(),
                *x_map.get(&x2).unwrap(),
                *y_map.get(&y2).unwrap(),
                &mut is_filled,
            ) {
                champ = cmp::max(champ, (x2 - x1 + 1) * (y2 - y1 + 1))
            }
        }
    }
    println!("{champ}");
}
