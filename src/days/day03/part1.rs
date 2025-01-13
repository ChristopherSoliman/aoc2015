use std::collections::HashSet;

const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut houses: HashSet<(i32, i32)> = HashSet::new();
    houses.insert((0, 0));

    let mut current: (i32, i32) = (0, 0);
    for c in input.trim().chars() {
        let idx: usize = match c {
            '^' => 0,
            '>' => 1,
            'v' => 2,
            '<' => 3,
            _ => unreachable!("invalid character"),
        };

        current = (
            current.0 + DIRS[idx].0 as i32,
            current.1 + DIRS[idx].1 as i32,
        );
        houses.insert(current);
    }

    houses.len()
}
