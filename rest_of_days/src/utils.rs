use tabled::{Tabled};

#[derive(Tabled)]
pub struct SolutionsTableRows {
    pub day: usize,
    pub part_1: String,
    pub part_2: String
}