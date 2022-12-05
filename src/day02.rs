// #![allow(warnings)]

pub fn run(input: &str) {
    let mut total_pt1 = 0;
    let mut total_pt2 = 0;
    for line in input.lines() {
        total_pt1 += round_score_part1(&line);
        total_pt2 += round_score_part2(&line);
    }
    println!("Pt 1 total score: {}", total_pt1);
    println!("Pt 2 total score: {}", total_pt2);
}

fn round_score_part1(guide: &str) -> i32 {
    let p1 = guide.chars().nth(0).unwrap();
    let p2 = guide.chars().nth(2).unwrap();
    let p1 = move_to_num(p1);
    let p2 = move_to_num(p2);
    let r = (p2 - p1).rem_euclid(3);
    let score = p2 + 1 + match r {2 => 0,
                                  0 => 3,
                                  1 => 6,
                                  _ => panic!()};
    score
}

fn round_score_part2(guide: &str) -> i32 {
    let p1 = guide.chars().nth(0).unwrap();
    let p2 = guide.chars().nth(2).unwrap();
    let p1 = move_to_num(p1);
    let win_lose = move_to_num(p2);
    let p2 = (p1 + win_lose - 1).rem_euclid(3);
    let score = p2 + 1 + match win_lose {0 => 0,
                                         1 => 3,
                                         2 => 6,
                                         _ => panic!()};
    score
}

fn move_to_num(m: char) -> i32 {
    match m {'A' | 'X' => 0,
             'B' | 'Y' => 1,
             'C' | 'Z' => 2,
             _ => panic!()}
}
