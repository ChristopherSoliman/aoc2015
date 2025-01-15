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
    let mut poss = 0;
    for i in 1..=iterations {
        let mut total = 0;
        for j in 0..containers.len() {
            if i & (1 << j) != 0 {
                total += containers[j];
            }
        }
        if total == 150 {
            poss += 1;
        }
    }
    poss
}
