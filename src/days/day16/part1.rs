pub fn run(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let required: Vec<(&str, u32)> = vec![
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ];

    for line in input.lines() {
        let (sue, props) = line.split_once(":").unwrap();
        let props = props
            .split(",")
            .into_iter()
            .map(|p| {
                let (n, v) = p.split_once(":").unwrap();
                (n.trim(), v.trim().parse::<u32>().unwrap())
            })
            .collect::<Vec<_>>();

        let mut bad = false;

        'outer_loop: for req in &required {
            for prop in &props {
                if req.0 == prop.0 {
                    if req.1 != prop.1 {
                        bad = true;
                        break 'outer_loop;
                    }
                    break;
                }
            }
        }
        if !bad {
            return sue.split_once(" ").unwrap().1.parse::<u32>().unwrap();
        }
    }

    0
}
