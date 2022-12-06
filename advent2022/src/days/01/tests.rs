use anyhow::Result;
use crate::days::one::{part_one, part_two};

#[test]
pub fn part_one_solution() -> Result<()> {
    assert_eq!(
        68802,
        part_one(group_by_lines())?
    );
    Ok(())
}

fn group_by_lines() -> Vec<Vec<u32>> {
    return include_str!("resources/input.txt")
        .split("\n\n")
        .map(|item| {
            item.split("\n")
                .map(|weight| weight.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
}

#[test]
pub fn part_two_solution() -> Result<()> {
    assert_eq!(
        205370,
        part_two(group_by_lines())?
    );
    Ok(())
}