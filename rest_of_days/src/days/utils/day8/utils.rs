pub type Matrix<T> = Vec<Vec<T>>;

#[derive(Debug)]
pub struct TreesMatrix {
    pub matrix: Matrix<u32>
}

#[derive(Copy, Clone, Debug)]
pub struct Tree {
    pub x: usize,
    pub y: usize,
    pub value: u32
}

impl TreesMatrix {
    pub fn move_up(&self, tree: Tree) -> Tree {
        Tree {
            x: tree.x,
            y: tree.y + 1,
            value: self.matrix[tree.y + 1][tree.x]
        }
    }
    pub fn move_down(&self, tree: Tree) -> Tree {
        Tree {
            x: tree.x,
            y: tree.y - 1,
            value: self.matrix[tree.y - 1][tree.x]
        }
    }
    pub fn move_left(&self, tree: Tree) -> Tree {
        Tree {
            x: tree.x - 1,
            y: tree.y,
            value: self.matrix[tree.y][tree.x - 1]
        }
    }
    pub fn move_right(&self, tree: Tree) -> Tree {
        Tree {
            x: tree.x + 1,
            y: tree.y,
            value: self.matrix[tree.y][tree.x + 1]
        }
    }
}

impl TreesMatrix {
    pub fn x_size(&self) -> usize {
        self.matrix[0].len()
    }
    pub fn y_size(&self) -> usize {
        self.matrix.len()
    }
}

impl Tree {
    pub fn visible_up(&self, trees_matrix: &TreesMatrix) -> (bool, usize) {
        let mut curr_tree = *self;
        let mut count: usize = 0;
        for _ in self.y..trees_matrix.y_size()-1 {
            let tree_above = trees_matrix.move_up(curr_tree);
            count += 1;
            if tree_above.value >= self.value {
                return (false, count)
            } else {
                curr_tree = tree_above
            }
        }
        (true, count)
    }
    pub fn visible_down(&self, trees_matrix: &TreesMatrix) -> (bool, usize) {
        let mut curr_tree = *self;
        let mut count: usize = 0;
        for _ in 0..self.y {
            let tree_below = trees_matrix.move_down(curr_tree);
            count += 1;
            if tree_below.value >= self.value {
                return (false, count)
            } else {
                curr_tree = tree_below
            }
        }
        (true, count)
    }
    pub fn visible_left(&self, trees_matrix: &TreesMatrix) -> (bool, usize) {
        let mut curr_tree = *self;
        let mut count: usize = 0;
        for _ in 0..self.x {
            let tree_above = trees_matrix.move_left(curr_tree);
            count += 1;
            if tree_above.value >= self.value {
                return (false, count)
            } else {
                curr_tree = tree_above
            }
        }
        (true, count)
    }
    pub fn visible_right(&self, trees_matrix: &TreesMatrix) -> (bool, usize) {
        let mut curr_tree = *self;
        let mut count: usize = 0;
        for _ in self.x..trees_matrix.x_size()-1 {
            let tree_above = trees_matrix.move_right(curr_tree);
            count += 1;
            if tree_above.value >= self.value {
                return (false, count)
            } else {
                curr_tree = tree_above
            }
        }
        (true, count)
    }
}