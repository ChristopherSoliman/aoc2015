struct Reindeer {
    points: u32,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Reindeer {
    pub fn get_distance(&self, time: u32) -> u32 {
        let mut cycles = time / (self.fly_time + self.rest_time);
        let mut rem = 0;
        if time % (self.fly_time + self.rest_time) > self.fly_time {
            cycles += 1
        } else {
            rem = time % (self.fly_time + self.rest_time);
        }
        self.speed * (self.fly_time * cycles + rem)
    }
}

const TIME: u32 = 2503;

pub fn run(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let mut reindeer = vec![];
    for line in input.lines() {
        let spl = line.split_whitespace().collect::<Vec<_>>();
        reindeer.push(Reindeer {
            points: 0,
            speed: spl[3].parse::<u32>().unwrap(),
            fly_time: spl[6].parse::<u32>().unwrap(),
            rest_time: spl[13].parse::<u32>().unwrap(),
        });
    }

    for i in 1..=TIME {
        let distances = reindeer
            .iter()
            .map(|r| r.get_distance(i))
            .collect::<Vec<_>>();
        let max = distances.iter().max().unwrap();

        for i in 0..reindeer.len() {
            if distances[i] == *max {
                reindeer[i].points += 1;
            }
        }
    }

    reindeer.iter().map(|r| r.points).max().unwrap()
}
