use std::collections::HashSet;

pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let weights = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let weight_per_compartment = weights.iter().sum::<usize>() / 4;

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
            //if curr.len() > min {
            //    break;
            //} else {
            //if is_possible(&curr, &weights, &weight_per_compartment, 1) {
            if !options.contains(&curr) {
                options.push(curr.clone());
                println!("{:?}", options);
            }
            //min = curr.len();
            //}
            //}
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
        q.sort_by_key(|v| v.len());
    }
    println!("{:?}", options);
    options
        .iter()
        .map(|v| v.iter().product::<usize>())
        .min()
        .unwrap()
}

fn is_possible(group: &Vec<usize>, weights: &Vec<usize>, target: &usize, level: usize) -> bool {
    if level == 4 {
        return group.iter().sum::<usize>() == *target;
    }

    let new_weights = weights
        .clone()
        .into_iter()
        .filter(|w| !group.contains(w))
        .collect::<Vec<_>>();

    let mut seen: HashSet<Vec<usize>> = HashSet::new();
    //let mut options = vec![];
    let mut q = new_weights
        .clone()
        .into_iter()
        .map(|v| vec![v])
        .collect::<Vec<_>>();

    while !q.is_empty() {
        let curr = q.pop().unwrap();
        let curr_sum = curr.iter().sum::<usize>();
        seen.insert(curr.clone());
        if curr_sum == *target {
            println!("New Weight: {:?}", new_weights);
            println!("{:?}", curr);
            if is_possible(&curr, &new_weights, &target, level + 1) {
                return true;
            }
            //options.push(curr.clone());
            //println!("{:?}", options);
            continue;
        }
        for weight in &new_weights {
            if curr_sum + weight > *target || curr.contains(&weight) {
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
        q.sort_by_key(|v| v.len());
    }
    //println!("{:?}", options);
    false
}
