pub fn run(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    input
        .trim()
        .chars()
        .map(|c| {
            if c == '(' {
                return 1;
            } else {
                return -1;
            }
        })
        .sum()
}
