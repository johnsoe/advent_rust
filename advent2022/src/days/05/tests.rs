use anyhow::Result;
use crate::days::five::{part_one, part_two};

#[test]
pub fn part_one_solution() -> Result<()> {
    let start_pos = generate_start_positions();
    let instructions = generate_instructions();
    part_one(instructions, start_pos);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let start_pos = generate_start_positions();
    let instructions = generate_instructions();
    part_two(instructions, start_pos);
    Ok(())
}

fn generate_start_positions() -> Vec<String> {
    return include_str!("resources/input_start_position.txt")
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|str| str.to_string())
        .collect();
}

fn generate_instructions() -> Vec<(u32, u32, u32)> {
    return include_str!("resources/input_instructions.txt")
        .lines()
        .map(|instruction| {
            let split = instruction.split_whitespace().collect::<Vec<&str>>();
            (split[1].parse::<u32>().unwrap(),
             split[3].parse::<u32>().unwrap() - 1,
             split[5].parse::<u32>().unwrap() - 1)
        })
        .collect::<Vec<(u32, u32, u32)>>();
}