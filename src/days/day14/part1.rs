struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Reindeer {
    pub fn get_distance(&self, time: u32) -> u32 {
        let mut cycles = time / (self.fly_time + self.rest_time);
        let mut rem = 0;
        if TIME % (self.fly_time + self.rest_time) >= self.fly_time {
            cycles += 1
        } else {
            rem = TIME % (self.fly_time + self.rest_time);
        }
        self.speed * (self.fly_time * cycles + rem)
    }
}

const TIME: u32 = 2503;

pub fn run(path: &str) -> u32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    input
        .lines()
        .map(|line| {
            let spl = line.split_whitespace().collect::<Vec<_>>();
            let reindeer = Reindeer {
                speed: spl[3].parse::<u32>().unwrap(),
                fly_time: spl[6].parse::<u32>().unwrap(),
                rest_time: spl[13].parse::<u32>().unwrap(),
            };
            reindeer.get_distance(TIME)
        })
        .max()
        .unwrap()
}
