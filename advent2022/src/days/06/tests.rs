use anyhow::Result;
use crate::days::six::{part_one};

#[test]
pub fn part_one_solution() -> Result<()> {
    let index = part_one(include_str!("resources/input.txt"), 4);
    println!("{}", index);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let index = part_one(include_str!("resources/input.txt"), 14);
    println!("{}", index);
    Ok(())
}