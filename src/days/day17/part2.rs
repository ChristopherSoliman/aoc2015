use std::collections::HashMap;

pub fn run(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let containers = input
        .lines()
        .map(|l| l.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut iterations = 0;
    for i in 0..containers.len() {
        iterations += 1 << i;
    }
    let mut combos: HashMap<u32, u32> = HashMap::new();

    for i in 1..=iterations {
        let mut total = 0;
        let mut used = 0;
        for j in 0..containers.len() {
            if i & (1 << j) != 0 {
                total += containers[j];
                used += 1;
            }
        }
        if total == 150 {
            combos.entry(used).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    *combos.iter().min_by_key(|(k, _)| *k).unwrap().1
}
