pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut sum = 0;
    for (i, c) in input.trim().chars().enumerate() {
        if c == '(' {
            sum += 1;
        } else {
            sum -= 1;
        }
        if sum == -1 {
            return i + 1;
        }
    }
    panic!("No solution found");
}
