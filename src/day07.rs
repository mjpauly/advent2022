#![allow(warnings)]

// Tree implementation adapted from:
// https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75

use std::cell::RefCell;
use std::rc::Rc;

struct DirNode {
    storage: u32, // space used by files at this directory level
    children: Vec<Rc<RefCell<DirNode>>>,
    parent: Option<Rc<RefCell<DirNode>>>,
}

impl DirNode {
    pub fn new() -> DirNode {
        return DirNode {
            storage: 0,
            children: vec![],
            parent: None,
        };
    }

    pub fn total_size(&self) -> u32 {
        return &self.storage
            + &self
                .children
                .iter()
                .map(|dn| dn.borrow().total_size())
                .sum();
    }

    pub fn total_in_dirs_less_than_100k(&self) -> u32 {
        let total = self.total_size();
        let to_add = if total < 100000 { total } else { 0 };
        return to_add
            + &self
                .children
                .iter()
                .map(|dn| dn.borrow().total_in_dirs_less_than_100k())
                .sum();
    }

    pub fn size_of_smallest_dir_greater_than(&self, thresh: u32) -> u32 {
        let total = self.total_size();
        let childbest = &self
            .children
            .iter()
            .map(|dn| dn.borrow().size_of_smallest_dir_greater_than(thresh))
            .filter(|x| *x >= thresh)
            .min();
        let best = match childbest {
            Some(x) => {
                if *x < total || total < thresh {
                    *x
                } else {
                    total
                }
            }
            None => total,
        };
        return best;
    }

    pub fn print(&self) -> String {
        return String::from(&self.storage.to_string())
            + "["
            + &self
                .children
                .iter()
                .map(|dn| dn.borrow().print())
                .collect::<Vec<String>>()
                .join(",")
            + "]";
    }
}

pub fn run(input: &str) {
    let root = Rc::new(RefCell::new(DirNode::new()));
    build_tree(input, &root);
    println!(
        "Total storage in directories less than 100k: {}",
        root.borrow().total_in_dirs_less_than_100k()
    );
    println!(
        "Size of smallest dir larger than threshold: {}",
        size_of_dir_to_delete(&root)
    );
}

fn size_of_dir_to_delete(root: &Rc<RefCell<DirNode>>) -> u32 {
    let space_needed = 30000000;
    let disk_total = 70000000;
    let space_used = root.borrow().total_size();
    let space_available = disk_total - space_used;
    let space_to_free = space_needed - space_available;
    return root
        .borrow()
        .size_of_smallest_dir_greater_than(space_to_free);
}

fn build_tree(input: &str, root: &Rc<RefCell<DirNode>>) {
    let mut current = Rc::clone(&root);
    for line in input.lines().skip(1) {
        // skip first line "cd /"
        if line.contains("dir ") {
            // skip dirs, add them when cd into them
            continue;
        } else if line.contains("$ ls") {
            // assume each ls happens only once, check that file storage is 0
            assert_eq!(current.borrow().storage, 0);
        } else if line.contains("$ cd ..") {
            // move up to parent
            let current_clone = Rc::clone(&current);
            current =
                Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
        } else if line.contains("$ cd ") {
            // go into to new child on cd
            // (assume cd's into directories happen exactly once)
            let child = Rc::new(RefCell::new(DirNode::new()));
            current.borrow_mut().children.push(Rc::clone(&child));
            {
                let mut mut_child = child.borrow_mut();
                mut_child.parent = Some(Rc::clone(&current));
            }
            current = child;
        } else {
            // add file size to current
            let filesize =
                line.split(' ').nth(0).unwrap().parse::<u32>().unwrap();
            current.borrow_mut().storage += filesize;
        }
    }
}
