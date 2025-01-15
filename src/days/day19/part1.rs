use std::collections::HashSet;

pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let (swaps, molecule) = input.split_once("\r\n\r\n").unwrap();
    let molecule = molecule.trim();

    let mut distinct: HashSet<String> = HashSet::new();

    for swap in swaps.lines() {
        let (a, b) = swap.split_once(" => ").unwrap();
        let (a, b) = (a.trim(), b.trim());
        for (i, _) in molecule.match_indices(a) {
            let (pre, suff) = molecule.split_at(i);
            let suff = suff.replacen(a, b, 1);
            let new_str = pre.to_string() + &suff[..];
            distinct.insert(new_str);
        }
    }
    distinct.iter().count()
}
