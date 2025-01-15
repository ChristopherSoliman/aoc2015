use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Eq, PartialEq)]
struct State {
    current: String,
    similarity: usize,
    swaps: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.similarity.cmp(&other.similarity))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.similarity.cmp(&other.similarity)
    }
}

pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let (swaps_list, molecule) = input.split_once("\r\n\r\n").unwrap();
    let molecule = molecule.trim();

    let mut swap_keys: HashSet<String> = HashSet::new();
    let swaps: Vec<(String, String)> = swaps_list
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(" => ").unwrap();
            swap_keys.insert(a.to_string());
            (a.trim().to_string(), b.trim().to_string())
        })
        .collect();

    let mut q = BinaryHeap::from(vec![State {
        current: "e".to_string(),
        similarity: 0,
        swaps: 0,
    }]);

    let mut seen: HashSet<(String, usize)> = HashSet::new();

    while let Some(state) = q.pop() {
        //if seen.len() % 100 == 0 {
        //    println!("{}", state.current);
        //}
        if state.similarity == molecule.len() {
            return state.swaps;
        }
        seen.insert((state.current.clone(), state.similarity));
        let (pre, suf) = state.current.split_at(state.similarity);

        // get possible swaps
        let mut swapable: Vec<(String, usize)> = vec![];
        let chars = suf.chars().collect::<Vec<_>>();
        let mut i = 0;
        while i < chars.len() {
            if chars[i].is_uppercase() || chars[i] == 'e' {
                let mut elem = String::from(chars[i]);
                if i + 1 < chars.len() && chars[i + 1].is_lowercase() {
                    elem.push(chars[i + 1]);
                    i += 1;
                }
                if swap_keys.contains(&elem) {
                    swapable.push((elem, i));
                }
            }
            i += 1;
        }

        //make swaps
        for s in &swapable {
            for poss in &swaps {
                if poss.0 == s.0 {
                    let (t_pre, t_suf) = suf.split_at(s.1);
                    let t_suf = t_suf.replacen(&poss.0, &poss.1, 1);
                    let n_str = pre.to_string() + &t_pre + &t_suf;
                    let sim = compare(&n_str, molecule);
                    if seen.contains(&(n_str.clone(), sim)) {
                        continue;
                    }
                    q.push(State {
                        current: n_str,
                        similarity: sim,
                        swaps: state.swaps + 1,
                    });
                }
            }
        }
    }
    panic!("no solution found");
}

pub fn compare(a: &str, b: &str) -> usize {
    let mut sum = 0;
    for i in 0..a.len() {
        if i >= b.len() {
            break;
        }
        if a.chars().nth(i) == b.chars().nth(i) {
            sum += 1;
        } else {
            break;
        }
    }
    sum
}
