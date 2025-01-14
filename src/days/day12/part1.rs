pub fn run(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).expect("File should be there");

    let mut sum = 0;
    let mut negative = false;
    let mut cons: Vec<u32> = vec![];

    for char in input.chars() {
        if char.is_digit(10) {
            cons.push(char.to_digit(10).unwrap());
        } else {
            if cons.is_empty() && char == '-' {
                negative = true;
            } else {
                if !cons.is_empty() {
                    let mut t: i32 = 0;
                    for i in 0..cons.len() {
                        t += (cons[i] * 10_u32.pow((cons.len() - i - 1) as u32)) as i32
                    }
                    if negative {
                        t *= -1;
                    }
                    sum += t;
                    cons.clear();
                }
                negative = false;
            }
        }
    }

    sum
}
