#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: u32,
}
pub fn run(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut ings = vec![];
    for line in input.lines() {
        let spl = line.trim().split_whitespace().collect::<Vec<_>>();
        ings.push(Ingredient {
            name: spl[0].trim().strip_suffix(":").unwrap().to_string(),
            capacity: spl[2].trim().strip_suffix(",").unwrap().parse().unwrap(),
            durability: spl[4].trim().strip_suffix(",").unwrap().parse().unwrap(),
            flavor: spl[6].trim().strip_suffix(",").unwrap().parse().unwrap(),
            texture: spl[8].trim().strip_suffix(",").unwrap().parse().unwrap(),
            calories: spl[10].trim().parse().unwrap(),
        });
    }

    let mut max = 0;
    for i in 1..=100 {
        for j in 1..=100 {
            for k in 1..=100 {
                let c = std::cmp::max(
                    ings[0].capacity * i
                        + ings[1].capacity * j
                        + ings[2].capacity * k
                        + ings[3].capacity * (100 - i - j - k),
                    0,
                );
                let d = std::cmp::max(
                    ings[0].durability * i
                        + ings[1].durability * j
                        + ings[2].durability * k
                        + ings[3].durability * (100 - i - j - k),
                    0,
                );
                let f = std::cmp::max(
                    ings[0].flavor * i
                        + ings[1].flavor * j
                        + ings[2].flavor * k
                        + ings[3].flavor * (100 - i - j - k),
                    0,
                );
                let t = std::cmp::max(
                    ings[0].texture * i
                        + ings[1].texture * j
                        + ings[2].texture * k
                        + ings[3].texture * (100 - i - j - k),
                    0,
                );
                let cl = std::cmp::max(
                    ings[0].calories * i as u32
                        + ings[1].calories * j as u32
                        + ings[2].calories * k as u32
                        + ings[3].calories * (100 - i - j - k) as u32,
                    0,
                );
                if cl != 500 {
                    continue;
                }
                let prod = c * d * f * t;
                if prod > max {
                    max = prod;
                }
            }
        }
    }

    max as u32
}
