use std::collections::{HashMap, HashSet};
use std::fs;

fn generate_rev_topo(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    seen: &mut HashSet<String>,
    order: &mut Vec<String>,
) {
    if seen.contains(node) || node == "out" {
        return;
    }

    seen.insert(node.to_string());

    for neighbor in graph.get(node).unwrap() {
        generate_rev_topo(neighbor, graph, seen, order)
    }

    order.push(node.to_string());
}

fn build_from_topo(rev_topo: &Vec<String>, graph: HashMap<String, Vec<String>>) -> u128 {
    let mut dp: HashMap<&str, (u128, u8)> = HashMap::new();
    dp.insert("out", (1, 0));
    dp.insert("dac", (0, 1));
    dp.insert("fft", (0, 2));

    for key in rev_topo {
        let (mut val, mut flag) = dp.get(key.as_str()).copied().unwrap_or((0, 0));

        let mut max_nflag = 0;

        for neighbor in graph.get(key).unwrap() {
            let (nkey, nflag) = dp.entry(neighbor).or_insert((0, 0));
            if max_nflag < *nflag{
                max_nflag = *nflag;
                val = 0;
            }
            // if any neighbors have a flag, only keep values that are neighbor flag | your flag
            if *nflag < max_nflag {
                continue;
            }
            val += *nkey;
            flag |= *nflag;
        }
        dp.insert(key.as_str(), (val, flag));
    }

    dp.get("svr").unwrap().0
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in content.lines() {
        let mut parts = line.split_whitespace();
        let key = parts.next().unwrap();
        let key = &key[..key.len() - 1]; // strip lagging :
        for neighbor in parts {
            graph
                .entry(key.to_string())
                .or_insert(Vec::new())
                .push(neighbor.to_string());
        }
    }

    let mut seen = HashSet::new();
    let mut rev_topo = Vec::new();

    generate_rev_topo("svr", &graph, &mut seen, &mut rev_topo);
    let routes = build_from_topo(&rev_topo, graph);
    dbg!(routes);
}
