// #![allow(warnings)]

pub fn run(input: &str) {
    let mut num_containing_pairs = 0;
    let mut num_partial_overlap = 0;
    for line in input.lines() {
        let nums: Vec<_> = line
                            .split(",")
                            .flat_map(|x| x.split("-"))
                            .map(|y| y.parse::<u32>().unwrap())
                            .collect();
        if let [a, b, c, d] = &nums[..] {
            if ((a >= c) && (b <= d)) ||
               ((a <= c) && (b >= d)) {
                num_containing_pairs += 1;
            }

            // cases for partial overlap:
            //
            //     a ---- b         a in between c and d
            // c ----- d
            //
            // a ---- b             b in between c and d
            //     c ----- d
            //
            // a ----- b            c in between a and b
            //   c - d
            if ((a >= c) && (a <= d)) ||
               ((b >= c) && (b <= d)) ||
               ((c >= a) && (c <= b)) {
                num_partial_overlap += 1;
            }
        } else {
            panic!("couldn't unpack nums from vec with slice patterns.");
        }
    }
    println!("Num pairs that have a full overlap: {}", num_containing_pairs);
    println!("Num pairs that have a parial overlap: {}", num_partial_overlap);
}
