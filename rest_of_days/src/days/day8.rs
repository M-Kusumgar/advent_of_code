use crate::{
    inputs::day8::{input_string},
    days::utils::day8::utils::{Matrix, Tree, TreesMatrix}
};

pub fn solve(part: i32) -> String {
    if part == 1 {
        part_1()
    } else if part == 2 {
        part_2()
    } else {
        "Stop trying to break it".to_owned()
    }
}

fn parser(input: &str) -> TreesMatrix {
    let clean_input: Matrix<u32> = input
        .lines()
        .map(|row| -> Vec<u32> {
            row.trim().chars()
            .map(|c| -> u32 {
                c.to_digit(10)
                .unwrap()
            }).collect()
        }).collect();
    TreesMatrix {
        matrix: clean_input
    }
}

fn part_1() -> String {
    let trees = parser(input_string());
    let mut visible_trees_count = 0;
    for (y_index, row) in trees.matrix.iter().enumerate() {
        for (x_index, tree) in row.iter().enumerate() {
            if x_index == trees.x_size()-1 || x_index == 0 {
                visible_trees_count += 1;
                continue;
            }
            if y_index == trees.y_size()-1 || y_index == 0 {
                visible_trees_count += 1;
                continue;
            }

            let current_tree = Tree {
                x: x_index,
                y: y_index,
                value: *tree
            };

            let (visible_up, _) = current_tree.visible_up(&trees);
            let (visible_down, _) = current_tree.visible_down(&trees);
            let (visible_left, _) = current_tree.visible_left(&trees);
            let (visible_right, _) = current_tree.visible_right(&trees);

            if visible_up || visible_down || visible_left || visible_right {
                visible_trees_count += 1;
            }
        }
    }
    visible_trees_count.to_string()
}

fn part_2() -> String {
    let trees = parser(input_string());
    let mut max_prod = 0;
    for (y_index, row) in trees.matrix.iter().enumerate() {
        for (x_index, tree) in row.iter().enumerate() {
            if x_index == trees.x_size()-1 || x_index == 0 {
                continue;
            }
            if y_index == trees.y_size()-1 || y_index == 0 {
                continue;
            }

            let current_tree = Tree {
                x: x_index,
                y: y_index,
                value: *tree
            };

            let (_, count_up) = current_tree.visible_up(&trees);
            let (_, count_down) = current_tree.visible_down(&trees);
            let (_, count_left) = current_tree.visible_left(&trees);
            let (_, count_right) = current_tree.visible_right(&trees);

            let curr_prod = count_down * count_left * count_right * count_up;
            if curr_prod > max_prod {
                max_prod = curr_prod;
            }
        }
    }

    max_prod.to_string()
}