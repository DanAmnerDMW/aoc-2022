use std::fs;

const INPUT_SIZE: usize = 14;

fn read_input_file() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn is_marker(input: &str) -> bool {
    for c in input.chars() {
        let mut char_count = 0;
        for test_char in input.chars() {
            if c.eq(&test_char) {
                char_count += 1
            }
            if char_count == 2 {
                return false;
            }
        }
    }

    true
}

fn main() {
    let file_input = read_input_file();

    let first_chars = &file_input[0..INPUT_SIZE];

    let mut marker = 0;

    if is_marker(first_chars) {
        marker = INPUT_SIZE;
    }

    for i in 1..=(file_input.len() - INPUT_SIZE) {
        if is_marker(&file_input[i..i + INPUT_SIZE]) {
            marker = i + INPUT_SIZE;
            break;
        }
    }

    println!("{}", marker);
}
