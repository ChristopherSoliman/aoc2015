use std::collections::HashMap;

pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");
    input.lines().filter(|l| is_nice(l)).count()
}

fn is_nice(a: &str) -> bool {
    let mut repeated = false;
    let mut between = false;

    let mut set: HashMap<(char, char), Vec<usize>> = HashMap::new();
    let mut prev: Option<char> = None;
    let mut prev_2: Option<char> = None;

    for (i, c) in a.chars().enumerate() {
        if let Some(p) = prev_2 {
            if p == c {
                between = true;
            }
        }
        if let Some(p) = prev {
            if let Some(starts) = set.get(&(p, c)) {
                for start in starts {
                    if *start != i - 1 {
                        repeated = true;
                    }
                }
            }
            set.entry((p, c))
                .and_modify(|v| v.push(i))
                .or_insert(vec![i]);
            prev_2 = Some(p);
        }
        prev = Some(c);
    }

    repeated && between
}
