pub fn run(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let chars = input.chars().collect::<Vec<_>>();

    let mut sum = 0;
    let mut i = 0;

    while i < chars.len() {
        let curr = chars[i];
        if curr.is_digit(10) || curr == '-' {
            let (num, dc) = get_number(&chars, i);
            sum += num;
            i += dc;
        }
        i += 1;
    }

    sum
}

fn get_number(chars: &Vec<char>, idx: usize) -> (i32, usize) {
    let negative = chars[idx] == '-';
    let mut vals: Vec<u32> = vec![];
    let mut i = if negative { idx + 1 } else { idx };

    while chars[i].is_digit(10) {
        vals.push(chars[i].to_digit(10).unwrap());
        i += 1;
    }

    if vals.len() == 0 {
        return (0, 0);
    }

    let mut sum: i32 = 0;
    for k in 0..vals.len() {
        sum += (vals[k] as i32) * 10_i32.pow(vals.len() as u32 - 1 - k as u32);
    }

    if negative {
        sum *= -1;
    }

    (sum, i - idx)
}
