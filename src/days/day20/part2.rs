pub fn run() -> usize {
    let input = 29_000_000;
    let mut i = 1;
    loop {
        let presents = get_presents(i);
        if presents >= input {
            return i;
        }
        i += 1;
    }
}

fn get_presents(n: usize) -> usize {
    let sqrt_n = f32::sqrt(n as f32) as usize + 1;
    let mut sum = 0;
    for i in 1..sqrt_n {
        if n % i == 0 {
            if n / i == i {
                if n / i <= 50 {
                    sum += i;
                }
            } else {
                if n / i <= 50 {
                    sum += i;
                }
                if i <= 50 {
                    sum += n / i;
                }
            }
        }
    }
    sum * 11
}
