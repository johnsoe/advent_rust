use anyhow::{Result, anyhow};

#[cfg(test)]
mod tests;

pub fn part_one(input: &str) -> Result<i32> {
    Ok(
        input.chars().map(
            |c| match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
        ).sum()
    )
}

pub fn part_two(input: &str) -> Result<usize> {
    
    let index = input
        .chars()
        .scan(0, |total, c| {
            *total += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            Some(*total)
        })
        .position(|total| total < 0)
        .ok_or_else(|| anyhow!("never found negative value"));
    Ok(index? + 1)
}
