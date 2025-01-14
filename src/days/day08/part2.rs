pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");
    input.lines().map(|line| length_diff(&line)).sum()
}

fn length_diff(line: &str) -> usize {
    let code_length = line.len();
    let mut mem_length = line.len() + 4;

    let mut i = 1;
    let chars = line.chars().collect::<Vec<_>>();

    while i + 1 < chars.len() {
        if chars[i] == '\\' || chars[i] == '"' {
            mem_length += 1
        }
        i += 1;
    }

    mem_length - code_length
}
