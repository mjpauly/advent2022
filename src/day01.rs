pub fn run(input: &str) {
    let mut top_three = vec![0; 3];
    let mut cur_cals = 0;
    for line in input.lines() {
        if line.chars().count() == 0 {
            replace_smallest_if_bigger(&mut top_three, cur_cals);
            cur_cals = 0;
        } else {
            cur_cals += line.parse::<i32>().unwrap();
        }
    }
    replace_smallest_if_bigger(&mut top_three, cur_cals);

    println!(
        "Three elves with the most calories are each carrying: \n\
             {:?} calories respectively.",
        top_three
    );
    println!(
        "Collectively they are carrying {} calories in total.",
        top_three.iter().sum::<i32>()
    );
}

fn replace_smallest_if_bigger(top_three: &mut Vec<i32>, cur_cals: i32) {
    /* Replace the smallest element in the vector with cur_cals if
     * cur_cals is bigger.
     */
    let mut smallest_idx = 0;
    for i in 1..3 {
        if top_three[i] < top_three[smallest_idx] {
            smallest_idx = i;
        }
    }
    if cur_cals > top_three[smallest_idx] {
        top_three[smallest_idx] = cur_cals;
    }
}
