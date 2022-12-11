// #![allow(warnings)]

pub fn run(input: &str) {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    build_grid(&input, &mut grid);
    let num_visible = count_visible(&grid);
    println!("pt1: num trees visible: {}", num_visible);
    let best_score = best_scenic_score(&grid);
    println!("pt2: best scenic score: {}", best_score);
}

fn build_grid(input: &str, grid: &mut Vec<Vec<u8>>) {
    // build the grid of tree heights into a 2D vector
    for line in input.lines() {
        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        grid.push(row);
    }
}

fn count_visible(grid: &Vec<Vec<u8>>) -> u32 {
    // count the number of trees that are visible from the forest edge
    let mut num_visible = 0;
    for i in 0..(grid.len()) {
        for j in 0..(grid[0].len()) {
            if is_visible(&grid, i, j) {
                num_visible += 1;
            }
        }
    }
    num_visible
}

fn is_visible(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    // determine if a given tree is visible from the forest edge
    let height = grid[i][j];
    let north = &grid[..i].iter().map(|x| x[j]).collect::<Vec<u8>>();
    let east = &grid[i][(j + 1)..].to_vec();
    let south = &grid[(i + 1)..].iter().map(|x| x[j]).collect::<Vec<u8>>();
    let west = &grid[i][..j].to_vec();
    return is_taller(&north, height)
        || is_taller(&east, height)
        || is_taller(&south, height)
        || is_taller(&west, height);
}

fn is_taller(row: &Vec<u8>, height: u8) -> bool {
    // determine if the given tree height is taller than all trees in the
    // cardinal of row
    match row.iter().max() {
        Some(x) => height > *x,
        None => true,
    }
}

// part 2

fn best_scenic_score(grid: &Vec<Vec<u8>>) -> u32 {
    // find the best scenic score of all trees in the forest
    let mut best = 0;
    for i in 0..(grid.len()) {
        for j in 0..(grid[0].len()) {
            let curr_score = scenic_score(&grid, i, j);
            if curr_score > best {
                best = curr_score;
            }
        }
    }
    best
}

fn scenic_score(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> u32 {
    // calculate product of viewing distances in each direction from tree
    let height = grid[i][j];
    // get slices in each direction, in the order of increasing distance from
    // the tree under consideration
    let north = &grid[..i].iter().map(|x| x[j]).rev().collect::<Vec<u8>>();
    let east = &grid[i][(j + 1)..].to_vec();
    let south = &grid[(i + 1)..].iter().map(|x| x[j]).collect::<Vec<u8>>();
    let west = &grid[i][..j].iter().map(|x| *x).rev().collect::<Vec<u8>>();
    [&north, &east, &south, &west]
        .iter()
        .map(|d| direction_scenic_score(height, d))
        .product()
}

fn direction_scenic_score(height: u8, sightline_heights: &Vec<u8>) -> u32 {
    // calc distance that can be seen from a given tree until another tree of
    // equal or greater height, or the edge of the forest
    let mut view_distance = 0;
    for i in 0..(sightline_heights.len()) {
        view_distance += 1;
        if sightline_heights[i] >= height {
            break;
        }
    }
    view_distance
}
