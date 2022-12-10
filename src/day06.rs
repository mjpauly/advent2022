// #![allow(warnings)]

pub fn run(input: &str) {
    println!("pt1: {}", find_marker(input, 4));
    println!("pt1: {}", find_marker(input, 14));
}

fn find_marker(line: &str, marker_len: usize) -> usize {
    for i in marker_len..line.len() {
        let substr = &line[(i - marker_len)..i];
        if all_unique(substr) {
            return i;
        }
    }
    0
}

fn all_unique(line: &str) -> bool {
    for j in 0..(line.len()) {
        if line.matches(line.chars().nth(j).unwrap()).count() > 1 {
            return false;
        }
    }
    true
}
