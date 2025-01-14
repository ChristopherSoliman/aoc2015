use std::collections::{HashMap, HashSet};

pub fn run(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut nodes: Vec<&str> = vec![];
    let mut edges: HashMap<(&str, &str), u32> = HashMap::new();

    for line in input.lines() {
        let (dest, dist) = line.split_once(" = ").unwrap();
        let dist = dist.parse::<u32>().unwrap();

        let (from, to) = dest.split_once(" to ").unwrap();

        edges.insert((from, to), dist);
        edges.insert((to, from), dist);

        if !nodes.contains(&from) {
            nodes.push(from);
        }
        if !nodes.contains(&to) {
            nodes.push(to);
        }
    }

    nodes
        .iter()
        .map(|node| djikstra(node, &nodes, &edges))
        .min()
        .unwrap() as u32
}

fn djikstra(start: &str, nodes: &Vec<&str>, edges: &HashMap<(&str, &str), u32>) -> u32 {
    let mut seen: HashSet<(&str, Vec<&str>)> = HashSet::new();
    let mut q: Vec<(&str, Vec<&str>, u32)> = vec![(start, vec![start], 0)];
    let mut dist: HashMap<u32, u32> = HashMap::new();

    dist.insert(1, 0);

    while !q.is_empty() {
        let (loc, visited, steps) = q.remove(0);
        if visited.len() == nodes.len() {
            return steps;
        }
        seen.insert((loc, visited.clone()));

        for node in nodes {
            if seen.contains(&(node, visited.clone())) || visited.contains(node) {
                continue;
            }
            let d = edges.get(&(loc, node)).unwrap();
            let mut visited = visited.clone();
            let vlen = visited.len() as u32;
            visited.push(node);
            q.push((node, visited, d + steps));
            dist.entry(vlen)
                .and_modify(|v| {
                    if *v > *d + steps {
                        *v = *d + steps;
                    }
                })
                .or_insert(d + steps);
        }
        q.sort_by_key(|v| v.2);
    }
    panic!("no solution found");
}
