use rest_of_days::days::{day8, day9};
use rest_of_days::utils::SolutionsTableRows;
use tabled::Table;

fn main() {
    let days: Vec<fn(i32) -> String> = vec![
        day8::solve,
        day9::solve
    ];

    let mut solutions: Vec<SolutionsTableRows> = Vec::new();
    for (index, solve) in days.iter().enumerate() {
        solutions.push(
            SolutionsTableRows {
                day: index + 8,
                part_1: solve(1),
                part_2: solve(2)
            }
        )
    }
    println!("{}", Table::new(solutions))
}
