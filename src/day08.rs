// #![allow(warnings)]

type TreeHeight = u8;
type TreeRow = Vec<TreeHeight>;
type Forest = Vec<TreeRow>;

trait TreeRowMethods {
    // pt 1 methods
    fn all_shorter_than(&self, height: TreeHeight) -> bool;
    // pt 2 methods
    fn direction_scenic_score(&self, height: TreeHeight) -> u32;
}

trait ForestMethods {
    // general methods
    fn from_str(input: &str) -> Forest;
    fn height_of(&self, i: usize, j: usize) -> TreeHeight;
    fn get_cardinal_rows(&self, i: usize, j: usize) -> [TreeRow; 4];
    // pt 1 methods
    fn count_visible(&self) -> u32;
    fn tree_is_visible(&self, i: usize, j: usize) -> bool;
    // pt 2 methods
    fn best_scenic_score(&self) -> u32;
    fn scenic_score(&self, i: usize, j: usize) -> u32;
}

impl TreeRowMethods for TreeRow {
    fn all_shorter_than(&self, height: TreeHeight) -> bool {
        match self.iter().max() {
            Some(x) => height > *x,
            None => true,
        }
    }

    fn direction_scenic_score(&self, height: TreeHeight) -> u32 {
        // calc distance that can be seen from a given tree until another tree of
        // equal or greater height, or the edge of the forest
        let mut view_distance = 0;
        for i in 0..(self.len()) {
            view_distance += 1;
            if self[i] >= height {
                break;
            }
        }
        view_distance
    }
}

impl ForestMethods for Forest {
    fn from_str(input: &str) -> Forest {
        // generate the forest from the input file
        let mut grid = Forest::new();
        for line in input.lines() {
            let mut row = TreeRow::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as TreeHeight);
            }
            grid.push(row);
        }
        grid
    }

    fn height_of(&self, i: usize, j: usize) -> TreeHeight {
        self[i][j]
    }

    fn get_cardinal_rows(&self, i: usize, j: usize) -> [TreeRow; 4] {
        // gets a vector of all the TreeRows in each cardinal direction from
        // a given tree, ordered with increasing distance from the tree
        let n: TreeRow = self[..i].iter().map(|x| x[j]).rev().collect();
        let e: TreeRow = self[i][(j + 1)..].to_vec();
        let s: TreeRow = self[(i + 1)..].iter().map(|x| x[j]).collect();
        let w: TreeRow = self[i][..j].iter().map(|x| *x).rev().collect();
        [n, e, s, w]
    }

    fn count_visible(&self) -> u32 {
        // count the number of trees visible from the forest edge
        let mut num_visible = 0;
        for i in 0..(self.len()) {
            for j in 0..(self[0].len()) {
                if self.tree_is_visible(i, j) {
                    num_visible += 1;
                }
            }
        }
        num_visible
    }

    fn tree_is_visible(&self, i: usize, j: usize) -> bool {
        // determine if the tree at (i,j) is visible from the edge of the grid
        self.get_cardinal_rows(i, j)
            .iter()
            .map(|row| row.all_shorter_than(self.height_of(i, j)))
            .fold(false, |acc, x| acc || x) // or together results
    }

    fn best_scenic_score(&self) -> u32 {
        // find the best scenic score of all trees in the forest
        let mut best = 0;
        for i in 0..(self.len()) {
            for j in 0..(self[0].len()) {
                let curr_score = self.scenic_score(i, j);
                if curr_score > best {
                    best = curr_score;
                }
            }
        }
        best
    }

    fn scenic_score(&self, i: usize, j: usize) -> u32 {
        // calculate product of viewing distances in each direction from tree
        self.get_cardinal_rows(i, j)
            .iter()
            .map(|row| row.direction_scenic_score(self.height_of(i, j)))
            .product()
    }
}

pub fn run(input: &str) {
    let grid = Forest::from_str(&input);
    println!("pt1: num trees visible: {}", grid.count_visible());
    println!("pt2: best scenic score: {}", grid.best_scenic_score());
}
