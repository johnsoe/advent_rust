use anyhow::Result;
use crate::days::two::{part_one, part_two};

#[test]
pub fn part_one_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .map(|item| item.split_at(1))
        .map(|game| (game.0, game.1.trim()))
        .collect::<Vec<(&str, &str)>>();

    part_one(input);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .map(|item| item.split_at(1))
        .map(|game| (game.0, game.1.trim()))
        .collect::<Vec<(&str, &str)>>();

    part_two(input);
    Ok(())
}