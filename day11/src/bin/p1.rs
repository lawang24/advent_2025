use std::{fs};
use std::collections::{HashMap, VecDeque};

fn bfs(starting_node: &str, graph:  & HashMap<String,Vec<String>> ) -> u128{
    let mut queue = VecDeque::from([starting_node]);

    let mut answer = 0;

    while let Some(curr_node) =  queue.pop_back() {
        if curr_node == "out"{
            answer +=1;
            continue;
        }

        for neighbor in graph.get(curr_node).unwrap(){
            queue.push_back(neighbor);
        }

    }

    answer
}

fn main(){
    let content = fs::read_to_string("input.txt").unwrap();

    let mut graph: HashMap<String,Vec<String>> = HashMap::new();

    for line in content.lines(){
        let mut parts = line.split_whitespace();
        let key = parts.next().unwrap();
        let key = &key[..key.len()-1];
        graph.insert(key.to_string(), Vec::new());
        for neighbor in parts {
            graph.get_mut(key).unwrap().push(neighbor.to_string());
        }
    }

    let mut answer = 0;

    answer += bfs("you", &graph);

    dbg!(answer);
    
}