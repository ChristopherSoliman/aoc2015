pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let input = input
        .replace("Rn", "(")
        .replace("Ar", ")")
        .replace("Y", ",");

    let (swap_lines, molecule) = input.split_once("\r\n\r\n").unwrap();
    let molecule = molecule.trim();
    let swaps = swap_lines
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(" => ").unwrap();
            (a.trim().to_string(), b.trim().to_string())
        })
        .collect::<Vec<_>>();

    let groups = get_groups(molecule);
    let (_, steps) = simplify_group(molecule, &swaps);

    steps as usize + groups.len() - 1
}

fn simplify_group(group: &str, swaps: &Vec<(String, String)>) -> (String, u32) {
    let mut steps = 0;
    let mut groups = get_groups(group);

    for i in 0..groups.len() {
        if let Some(k) = groups[i].find("(") {
            let (pre, suf) = groups[i].split_at(k + 1);
            let suf = &suf[0..suf.len() - 1];
            let comma_groups = suf.split(",");
            let simplified = comma_groups
                .map(|cg| {
                    let (n_cg, ds) = simplify_group(&cg.to_string(), swaps);
                    steps += ds;
                    n_cg
                })
                .collect::<Vec<_>>();
            groups[i] = pre.to_string() + &simplified.join(",") + ")";
        }
    }

    while groups.len() > 1 {
        let (new_a, ds_a) = simplify_group(&groups.remove(0), swaps);
        let (new_b, ds_b) = simplify_group(&groups.remove(0), swaps);

        steps += ds_a + ds_b;
        let combo = new_a + &new_b;
        for swap in swaps {
            if swap.1 == combo {
                groups.insert(0, swap.0.clone());
                steps += 1;
                break;
            }
        }
    }

    (groups.join(","), steps)
}

fn get_groups(molecule: &str) -> Vec<String> {
    let m_chars = molecule.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut groups = vec![];
    let mut current_group = String::new();
    let mut bracket = 0;
    while i < m_chars.len() {
        if bracket == 0 && m_chars[i].is_uppercase() {
            if !current_group.is_empty() {
                groups.push(current_group.clone());
                current_group.clear();
            }
            current_group.push(m_chars[i]);
            if i + 1 < m_chars.len() && m_chars[i + 1].is_lowercase() {
                current_group.push(m_chars[i + 1]);
                i += 1;
            }
        } else {
            if m_chars[i] == '(' {
                bracket += 1;
            } else if m_chars[i] == ')' {
                bracket -= 1;
            }
            current_group.push(m_chars[i]);
            if bracket == 0 {
                groups.push(current_group.clone());
                current_group.clear();
            }
        }
        i += 1;
    }
    if !current_group.is_empty() {
        groups.push(current_group);
    }
    groups
}
