pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut lights: [[u8; 1000]; 1000] = [[0; 1000]; 1000];

    for line in input.lines() {
        let ws = line.split_whitespace().collect::<Vec<_>>();
        if ws[0] == "toggle" {
            let (si, sj) = ws[1].split_once(",").unwrap();
            let (ei, ej) = ws[3].split_once(",").unwrap();
            let (si, sj) = (si.parse::<usize>().unwrap(), sj.parse::<usize>().unwrap());
            let (ei, ej) = (ei.parse::<usize>().unwrap(), ej.parse::<usize>().unwrap());

            for i in si..=ei {
                for j in sj..=ej {
                    lights[i][j] += 2;
                }
            }
        } else {
            let (si, sj) = ws[2].split_once(",").unwrap();
            let (ei, ej) = ws[4].split_once(",").unwrap();
            let (si, sj) = (si.parse::<usize>().unwrap(), sj.parse::<usize>().unwrap());
            let (ei, ej) = (ei.parse::<usize>().unwrap(), ej.parse::<usize>().unwrap());

            if ws[1] == "on" {
                for i in si..=ei {
                    for j in sj..=ej {
                        lights[i][j] += 1;
                    }
                }
            } else {
                for i in si..=ei {
                    for j in sj..=ej {
                        if lights[i][j] > 0 {
                            lights[i][j] -= 1;
                        }
                    }
                }
            }
        }
    }

    lights
        .iter()
        .map(|r| r.iter().map(|l| *l as usize).sum::<usize>())
        .sum()
}
