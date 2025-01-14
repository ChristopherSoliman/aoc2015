const LOWER_A: u8 = 'a' as u8;
const LOWER_I: u8 = 'i' as u8 - LOWER_A;
const LOWER_L: u8 = 'l' as u8 - LOWER_A;
const LOWER_O: u8 = 'o' as u8 - LOWER_A;

pub fn run() -> String {
    let input = "cqjxjnds";
    let mut pw_vec = input.chars().map(|c| c as u8 - LOWER_A).collect::<Vec<_>>();

    loop {
        increment(&mut pw_vec);
        if valid(&pw_vec) {
            return pw_vec.iter().map(|v| (v + LOWER_A) as char).collect();
        }
    }
}

fn valid(a: &Vec<u8>) -> bool {
    let mut pairs_found = 0;
    let mut longest_run = 0;
    let mut current_run = 1;
    let mut prev = 30;
    let mut pairs: Vec<usize> = vec![];

    for i in 0..a.len() {
        let letter = a[i];
        if letter == prev + 1 {
            current_run += 1;
        } else {
            if current_run > longest_run {
                longest_run = current_run;
            }
            current_run = 1;
        }
        if letter == prev {
            if !pairs.contains(&(i - 1)) {
                pairs_found += 1;
            }
            pairs.push(i);
        }
        prev = letter;
    }

    longest_run >= 3 && pairs_found >= 2
}

fn increment(a: &mut Vec<u8>) {
    let length = a.len();
    a[length - 1] = (a[length - 1] + 1) % 26;
    let mut carry = a[length - 1] == 0;

    let mut i = length - 2;
    while carry && i > 0 {
        a[i] += 1;
        a[i] %= 26;
        carry = a[i] == 0;
        i -= 1;
    }

    let mut bad = false;
    for i in 1..length {
        if !bad && (a[i] == LOWER_I || a[i] == LOWER_L || a[i] == LOWER_O) {
            bad = true;
            a[i] += 1;
        } else {
            if bad {
                a[i] = 0;
            }
        }
    }
}
