pub fn run() -> usize {
    let input = 29_000_000;
    let mut i = 0;
    loop {
        let factors = get_factors(i);
        let presents = factors.iter().map(|v| v * 10).sum::<usize>();
        if presents >= input {
            return i;
        }
        i += 2;
    }
}

fn get_factors(n: usize) -> Vec<usize> {
    let mut factors = vec![];
    let mut i = 1;
    let sqrt_n = f32::sqrt(n as f32) as usize;
    while i <= sqrt_n {
        if n % i == 0 {
            if n / i == i {
                factors.push(i);
            } else {
                factors.push(i);
                factors.push(n / i);
            }
        }
        i += 1;
    }
    factors
}
