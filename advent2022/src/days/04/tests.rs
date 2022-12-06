use std::ops::Range;
use anyhow::Result;
use regex::Regex;
use crate::days::four::{part_one, part_two};

#[test]
pub fn part_one_solution() -> Result<()> {
    part_one(create_sections());
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    part_two(create_sections());
    Ok(())
}

fn create_sections() -> Vec<(Range<u32>, Range<u32>)> {
    let regex = Regex::new(r"[-,]").unwrap();
    return include_str!("resources/input.txt")
        .lines()
        .map(|item| regex.split(&item).collect::<Vec<&str>>())
        .map(|nums| {
            nums.iter()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|nums| (nums[0]..nums[1], nums[2]..nums[3]))
        .collect::<Vec<(Range<u32>, Range<u32>)>>();
}