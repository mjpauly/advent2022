// #![allow(warnings)]

use std::collections::HashSet;

pub fn run(input: &str) {
    let mut total_pt1 = 0;
    let mut total_pt2 = 0;
    let mut group: Vec<String> = vec![String::new(); 3];
    for (i, line) in input.lines().enumerate() {
        total_pt1 += shared_item_priority(&line);

        let nth_elf = i.rem_euclid(3);
        // group[nth_elf] = line.clone();
        group[nth_elf] = String::from(line);
        if nth_elf == 2 {
            total_pt2 += pt2_priority(&group);
        }
    }
    println!("Pt 1 priority: {}", total_pt1);
    println!("Pt 2 priority: {}", total_pt2);
}

fn shared_item_priority(rucksack: &str) -> u32 {
    let len = rucksack.chars().count();
    let a = pouch_contents(&rucksack[..(len / 2)]);
    let b = pouch_contents(&rucksack[(len / 2)..]);
    let intersection: Vec<_> = a.intersection(&b).collect();
    assert_eq!(intersection.len(), 1);
    let shared = intersection[0];
    score_item(*shared)
}

fn pouch_contents(pouch: &str) -> HashSet<char> {
    let mut pouch_contents: HashSet<char> = HashSet::new();
    pouch_contents.extend::<Vec<char>>(pouch.chars().collect());
    pouch_contents
}

fn score_item(item: char) -> u32 {
    let ord: u32 = item.into();
    let a: u32 = 'a'.into();
    let biga: u32 = 'A'.into();
    if item.is_uppercase() {
        return ord - biga + 1 + 26;
    }
    ord - a + 1
}

fn pt2_priority(group: &Vec<String>) -> u32 {
    let a = pouch_contents(group[0].as_str());
    let b = pouch_contents(group[1].as_str());
    let c = pouch_contents(group[2].as_str());
    let d: HashSet<char> = a.intersection(&b).copied().collect();
    let e: Vec<&char> = c.intersection(&d).collect();
    assert_eq!(e.len(), 1);
    let shared = e[0];
    score_item(*shared)
}
