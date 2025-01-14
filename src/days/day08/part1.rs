pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");
    input.lines().map(|line| length_diff(&line)).sum()
}

fn length_diff(line: &str) -> usize {
    let code_length = line.len();
    let mut mem_length = line.len() - 2;

    let mut i = 1;
    let chars = line.chars().collect::<Vec<_>>();

    while i + 1 < chars.len() {
        if chars[i] == '\\' {
            let k = match chars[i + 1] {
                '\\' | '"' => 1,
                'x' if i + 3 < chars.len() => 3,
                _ => 0,
            };
            mem_length -= k;
            i += k;
        }
        i += 1;
    }

    code_length - mem_length
}
