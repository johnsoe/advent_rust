use anyhow::Result;

use crate::days::two::{part_one, part_two};

#[test]
pub fn part_one_solution() -> Result<()> {
    let input: Vec<Vec<i32>> = include_str!("resources/input.txt")
        .lines()
        .map(|line| line.split('x').map(|d| d.parse().unwrap()).collect())
        .collect();

    assert_eq!(
        1588178i32,
        part_one(input)?,
    );
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input: Vec<Vec<i32>> = include_str!("resources/input.txt")
        .lines()
        .map(|line| line.split('x').map(|d| d.parse().unwrap()).collect())
        .collect();

    assert_eq!(
        3783758i32,
        part_two(input)?,
    );
    Ok(())
}