pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");
    input.lines().filter(|l| is_nice(l)).count()
}

const NAUGHTY: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn is_nice(a: &str) -> bool {
    let mut vowels = 0;
    let mut repeated = false;
    let mut contains_bad = false;

    let mut prev: Option<char> = None;
    for c in a.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
            _ => {}
        }
        if let Some(p) = prev {
            if p == c {
                repeated = true;
            } else if NAUGHTY.contains(&&format!("{p}{c}")[..]) {
                contains_bad = true;
            }
        }

        prev = Some(c);
    }

    vowels >= 3 && repeated && !contains_bad
}
