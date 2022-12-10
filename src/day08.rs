// #![allow(warnings)]

pub fn run(input: &str) {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    build_grid(&input, &mut grid);
    let num_visible = count_visible(&grid);
    println!("pt1: num trees visible: {}", num_visible);
}

fn build_grid(input: &str, grid: &mut Vec<Vec<u8>>) {
    for line in input.lines() {
        let mut row: Vec<u8> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        grid.push(row);
    }
}

fn count_visible(grid: &Vec<Vec<u8>>) -> u32 {
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
    match row.iter().max() {
        Some(x) => height > *x,
        None => true,
    }
}
