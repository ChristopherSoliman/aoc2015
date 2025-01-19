use std::collections::HashSet;

pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let weights = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .rev()
        .collect::<Vec<_>>();

    let weight_per_compartment = weights.iter().sum::<usize>() / 4;
    let mut seen: HashSet<Vec<usize>> = HashSet::new();
    let mut q = vec![vec![113]];

    while !q.is_empty() {
        let curr = q.remove(0);
        seen.insert(curr.clone());
        let curr_sum = curr.iter().sum::<usize>();
        if curr_sum == weight_per_compartment {
            return curr.iter().product::<usize>();
        }
        for weight in &weights {
            if curr_sum + weight > weight_per_compartment || curr.contains(&weight) {
                continue;
            }
            let mut v = curr.clone();
            v.push(*weight);
            v.sort();
            if seen.contains(&v) {
                continue;
            }
            q.push(v);
        }
    }
    panic!("No solution found");
}
