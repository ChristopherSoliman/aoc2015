pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut lights = [[false; 1000]; 1000];

    for line in input.lines() {
        let ws = line.split_whitespace().collect::<Vec<_>>();
        if ws[0] == "toggle" {
            let (si, sj) = ws[1].split_once(",").unwrap();
            let (ei, ej) = ws[3].split_once(",").unwrap();
            let (si, sj) = (si.parse::<usize>().unwrap(), sj.parse::<usize>().unwrap());
            let (ei, ej) = (ei.parse::<usize>().unwrap(), ej.parse::<usize>().unwrap());

            for i in si..=ei {
                for j in sj..=ej {
                    lights[i][j] = !lights[i][j];
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
                        lights[i][j] = true;
                    }
                }
            } else {
                for i in si..=ei {
                    for j in sj..=ej {
                        lights[i][j] = false;
                    }
                }
            }
        }
    }

    lights
        .iter()
        .map(|r| r.iter().filter(|v| **v).count())
        .sum()
}
