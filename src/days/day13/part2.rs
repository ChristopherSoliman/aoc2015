use std::collections::HashMap;

pub fn run(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut nodes: Vec<&str> = vec![];
    let mut edges: HashMap<(&str, &str), i32> = HashMap::new();

    for line in input.lines() {
        let spl = line
            .strip_suffix(".")
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();
        let dh = match (spl[2], spl[3]) {
            ("lose", v) => -v.parse::<i32>().unwrap(),
            ("gain", v) => v.parse::<i32>().unwrap(),
            _ => unreachable!("invalid string"),
        };

        if !nodes.contains(&spl[0].trim()) {
            nodes.push(spl[0].trim());
        }
        edges.insert((spl[0].trim(), spl[10].trim()), dh);
    }

    for node in &nodes {
        edges.insert(("Me", node), 0);
        edges.insert((node, "Me"), 0);
    }
    nodes.push("Me");

    djikstra(nodes[0], &nodes, &edges)
}

fn djikstra(start: &str, nodes: &Vec<&str>, edges: &HashMap<(&str, &str), i32>) -> i32 {
    let mut q: Vec<(&str, Vec<&str>, i32)> = vec![(start, vec![start], 0)];
    let mut dist: HashMap<usize, i32> = HashMap::new();

    dist.insert(1, 0);
    while !q.is_empty() {
        let (loc, visited, steps) = q.remove(0);
        if visited.len() == nodes.len() {
            continue;
        }
        for node in nodes {
            if *node == loc || visited.contains(node) {
                continue;
            }
            let mut d = edges.get(&(loc, node)).unwrap() + edges.get(&(node, loc)).unwrap();
            let mut visited = visited.clone();
            visited.push(node);
            if visited.len() == nodes.len() {
                d += edges.get(&(start, node)).unwrap() + edges.get(&(node, start)).unwrap();
            }
            let vlen = visited.len();
            q.push((node, visited, d + steps));
            dist.entry(vlen)
                .and_modify(|v| {
                    if *v < d + steps {
                        *v = d + steps;
                    }
                })
                .or_insert(d + steps);
        }
        q.sort_by(|a, b| b.2.cmp(&a.2));
    }

    *dist.get(&nodes.len()).unwrap()
}
