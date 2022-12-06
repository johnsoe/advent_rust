use std::collections::BTreeSet;
use anyhow::Result;

#[cfg(test)]
mod tests;


pub fn part_one(calories: Vec<Vec<u32>>) -> Result<u32> {
    Ok(calories
        .iter()
        .map(|group| group.iter().sum())
        .max()
        .unwrap())
}

pub fn part_two(calories: Vec<Vec<u32>>) -> Result<u32> {
    Ok(calories
        .iter()
        .map(|group| group.iter().sum())
        .collect::<BTreeSet<u32>>()
        .iter()
        .rev()
        .take(3)
        .sum())
}