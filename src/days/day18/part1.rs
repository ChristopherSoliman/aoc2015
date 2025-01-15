const DIRS: [(i8, i8); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

const N: usize = 100;

pub fn run(path: &str) -> usize {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut lights = [[false; N]; N];

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                lights[i][j] = true;
            }
        }
    }

    for _ in 0..100 {
        lights = animate(&lights);
    }

    lights
        .iter()
        .map(|r| r.iter().filter(|l| **l).count())
        .sum()
}

fn animate(lights: &[[bool; N]; N]) -> [[bool; N]; N] {
    let mut n_lights = lights.clone();

    for i in 0..N {
        for j in 0..N {
            let neighbours = get_neighbours(lights, i, j);
            if lights[i][j] && !(neighbours == 2 || neighbours == 3) {
                n_lights[i][j] = false;
            }
            if !lights[i][j] && neighbours == 3 {
                n_lights[i][j] = true;
            }
        }
    }
    n_lights
}

fn get_neighbours(lights: &[[bool; N]; N], i: usize, j: usize) -> u32 {
    let mut neighbours = 0;
    for dir in DIRS {
        let (nr, nc) = (i as i32 + dir.0 as i32, j as i32 + dir.1 as i32);
        if nr < 0 || nr >= N as i32 || nc < 0 || nc >= N as i32 {
            continue;
        }
        let (nr, nc) = (nr as usize, nc as usize);
        if lights[nr][nc] {
            neighbours += 1;
        }
    }
    neighbours
}
