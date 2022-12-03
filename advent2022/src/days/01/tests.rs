use anyhow::Result;
use crate::days::one::{part_one, part_two};

#[test]
pub fn part_one_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .split("\n\n")
        .map(|item| {
            item.split("\n")
                .map(|weight| weight.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();


    part_one(input);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .split("\n\n")
        .map(|item| {
            item.split("\n")
                .map(|weight| weight.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    part_two(input);
    Ok(())
}