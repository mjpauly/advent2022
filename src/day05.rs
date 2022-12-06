// #![allow(warnings)]

pub fn run(input: &str) {
    let lines: Vec<_> = input.lines().collect();

    // determine placement of first blank line
    let mut blank_idx = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.len() == 0 {
            blank_idx = i;
        }
    }

    // build up stacks with vectors
    let num_stacks = lines[blank_idx - 2].split(" ").collect::<Vec<_>>().len();
    // front of vectors will be the bottom of the stack
    // push/pop will be from the end of the vector
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];
    for i in (0..(blank_idx - 1)).rev() {  // bottom to top of stacks
        for j in 0..num_stacks {  // at each level, add to each stack
            let topush = lines[i].chars().nth(j*4 + 1).unwrap();
            if topush != ' ' {
                stacks[j].push(topush);
            }
        }
    }

    let mut stacks2 = stacks.clone();  // for part 2

    // follow crane operations
    for line in lines[(blank_idx + 1)..].iter() {
        let words = line.split(" ").collect::<Vec<_>>();
        let n_to_move = words[1].parse::<usize>().unwrap();
        let src = words[3].parse::<usize>().unwrap() - 1;
        let dst = words[5].parse::<usize>().unwrap() - 1;
        for _i in 0..n_to_move {  // pt1
            let c = stacks[src].pop().unwrap();
            stacks[dst].push(c);
        }
        // pt2
        let stack_height = stacks2[src].len();
        let mut slice = stacks2[src].split_off(stack_height - n_to_move);
        stacks2[dst].append(&mut slice);
    }

    // print out the crates on top at the end
    print!("Crates on top (pt1): ");
    print_top_stacks(&stacks, num_stacks);
    print!("Crates on top (pt2): ");
    print_top_stacks(&stacks2, num_stacks);
}

fn print_top_stacks(stacks: &Vec<Vec<char>>, num_stacks: usize) {
    for i in 0..num_stacks {
        print!("{}", stacks[i].last().unwrap());
    }
    println!();
}
