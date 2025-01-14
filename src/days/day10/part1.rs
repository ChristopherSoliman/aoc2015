pub fn run() -> usize {
    let mut input = "1113222113".to_string();
    for _ in 0..40 {
        input = look_and_say(&input);
    }
    input.len()
}

fn look_and_say(a: &String) -> String {
    let mut prev: Option<char> = None;

    let mut counter = 1;
    let mut new_str = String::new();
    for char in a.chars() {
        if let Some(p) = prev {
            if p == char {
                counter += 1;
            } else {
                new_str += &format!("{counter}{p}")[..];
                counter = 1;
            }
        }
        prev = Some(char);
    }
    new_str += &format!("{counter}{}", prev.unwrap())[..];
    new_str
}
