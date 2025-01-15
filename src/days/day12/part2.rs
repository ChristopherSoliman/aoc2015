pub fn run(path: &str) -> i32 {
    let input = std::fs::read_to_string(path).expect("File should be there");
    let chars = input.chars().collect::<Vec<_>>();
    get_level_sum(&chars, 0).0
}

fn get_level_sum(chars: &Vec<char>, start: usize) -> (i32, usize) {
    let mut level = 0;
    if chars[start] != '{' {
        return (0, 0);
    }
    let mut bad_level = false;
    let mut sum = 0;
    let mut i = start + 1;

    loop {
        let curr = chars[i];
        if curr == '{' {
            level += 1;
            if !bad_level {
                let (level_sum, dc) = get_level_sum(chars, i);
                sum += level_sum;
                i += dc - 1;
            }
        } else if curr == '}' {
            if level == 0 {
                break;
            }
            level -= 1;
        }
        if !bad_level {
            if curr.is_digit(10) || curr == '-' {
                let (num, dc) = get_number(&chars, i);
                sum += num;
                i += dc - 1;
            }
            if curr == ':' {
                if found_red_object(&chars, i) {
                    bad_level = true;
                    sum = 0;
                }
            }
        }
        i += 1;
    }

    (sum, i - start)
}

fn found_red_object(chars: &Vec<char>, idx: usize) -> bool {
    if idx + 5 > chars.len() {
        return false;
    }
    return chars[idx] == ':'
        && chars[idx + 1] == '"'
        && chars[idx + 2] == 'r'
        && chars[idx + 3] == 'e'
        && chars[idx + 4] == 'd'
        && chars[idx + 5] == '"';
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
