use anyhow::Result;
use crate::days::one::{part_one, part_two};

#[test]
pub fn part_one_solution() -> Result<()> {
    let input: Vec<u32> = include_str!("resources/input.txt")
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    part_one(&input);
    Ok(())
}

#[test]
pub fn part_one_test() -> Result<()> {
    let input: Vec<u32> = include_str!("resources/test.txt")
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    part_one(&input);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input: Vec<u32> = include_str!("resources/input.txt")
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    part_two(&input);
    Ok(())
}