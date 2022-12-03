use anyhow::Result;
use crate::days::two::{part_one, part_two};

#[test]
pub fn part_one_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .map(|row| {
            row.split_whitespace()
                .flat_map(|item|item.parse::<u32>())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    part_one(&input);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .map(|row| {
            row.split_whitespace()
                .flat_map(|item|item.parse::<u32>())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    part_two(&input);
    Ok(())
}