use anyhow::Result;
use itertools::Itertools;
use crate::days::three::{part_one, part_two};

#[test]
pub fn part_one_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .map(|item| item.split_at(item.len()/2))
        .collect::<Vec<(&str, &str)>>();

    println!("{:?}", input);
    part_one(input);
    Ok(())
}

#[test]
pub fn part_one_test_solution() -> Result<()> {
    let input = include_str!("resources/test.txt")
        .lines()
        .map(|item| item.split_at(item.len()/2))
        .collect::<Vec<(&str, &str)>>();

    println!("{:?}", input);
    part_one(input);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]))
        .collect::<Vec<(&str, &str, &str)>>();

    part_two(input);
    Ok(())
}