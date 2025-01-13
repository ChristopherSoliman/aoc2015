use std::collections::HashSet;

const DIRS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut houses: HashSet<(i32, i32)> = HashSet::new();
    houses.insert((0, 0));

    let mut santa_pos: (i32, i32) = (0, 0);
    let mut robo_pos: (i32, i32) = (0, 0);
    for (i, c) in input.trim().chars().enumerate() {
        let idx: usize = match c {
            '^' => 0,
            '>' => 1,
            'v' => 2,
            '<' => 3,
            _ => unreachable!("invalid character"),
        };

        if i % 2 == 1 {
            santa_pos = (
                santa_pos.0 + DIRS[idx].0 as i32,
                santa_pos.1 + DIRS[idx].1 as i32,
            );
            houses.insert(santa_pos);
        } else {
            robo_pos = (
                robo_pos.0 + DIRS[idx].0 as i32,
                robo_pos.1 + DIRS[idx].1 as i32,
            );
            houses.insert(robo_pos);
        }
    }

    houses.len()
}
