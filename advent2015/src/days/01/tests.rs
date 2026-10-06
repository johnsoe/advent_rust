use anyhow::Result;

use crate::days::one::{part_one, part_two};

#[test]
pub fn part_one_solution() -> Result<()> {
    let input: &str = include_str!("resources/input.txt");
    assert_eq!(
        232,
        part_one(input)?,
    );
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input: &str = include_str!("resources/input.txt");
    assert_eq!(
        1783,
        part_two(input)?,
    );
    Ok(())
}