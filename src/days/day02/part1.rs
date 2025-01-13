pub fn run(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    input
        .lines()
        .map(|line| {
            let mut dims = line
                .split("x")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            dims.sort();
            dims[0] * dims[1] * 2
                + dims[1] * dims[2] * 2
                + dims[0] * dims[2] * 2
                + dims[0] * dims[1]
        })
        .sum()
}
