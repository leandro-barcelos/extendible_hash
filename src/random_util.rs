use rand::{thread_rng, Rng};

pub fn unique_random_numbers(start: i32, end: i32) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut numbers: Vec<i32> = (start..=end).collect();

    for i in (start..=end).rev() {
        let j = rng.gen_range(start..i + 1);
        numbers.swap((i - start) as usize, (j - start) as usize);
    }

    numbers
}

pub fn random_string(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    let mut string = String::new();
    for _ in 0..len {
        let idx = rng.gen_range(0..chars.len());
        string.push(chars[idx]);
    }
    string
}
