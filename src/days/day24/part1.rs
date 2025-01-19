use std::collections::HashSet;

pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let weights = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let weight_per_compartment = weights.iter().sum::<usize>() / 3;

    let mut q = weights
        .clone()
        .into_iter()
        .map(|v| vec![v])
        .collect::<Vec<_>>();

    let mut options = vec![];
    let mut min = usize::MAX;
    let mut seen: HashSet<Vec<usize>> = HashSet::new();

    while !q.is_empty() {
        let curr = q.pop().unwrap();
        let curr_sum = curr.iter().sum::<usize>();
        if curr_sum == weight_per_compartment {
            if curr.len() > min {
                break;
            } else {
                if !options.contains(&curr) {
                    options.push(curr.clone());
                }
                min = curr.len();
            }
            continue;
        }
        seen.insert(curr.clone());
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
    options
        .iter()
        .map(|v| v.iter().product::<usize>())
        .min()
        .unwrap()
}
