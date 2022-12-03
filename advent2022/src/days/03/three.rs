use std::collections::{HashSet};

#[cfg(test)]
mod tests;

pub fn part_one(rucksacks: Vec<(&str, &str)>) {
    let common: u32 = rucksacks.iter()
        .map(|&rucks| (
            rucks.0.chars().collect::<HashSet<char>>(),
            rucks.1.chars().collect::<HashSet<char>>()
        ))
        .map(|sets| {
            *sets.0.intersection(&sets.1)
                .next()
                .unwrap()
        })
        .map(|c| char_to_score(c))
        .sum();

    println!("{}", common);
}

pub fn part_two(rucksacks: Vec<(&str, &str, &str)>) {
    let common: u32 = rucksacks.iter()
        .map(|&sacks| (
            sacks.0.chars().collect::<HashSet<char>>(),
            sacks.1.chars().collect::<HashSet<char>>(),
            sacks.2.chars().collect::<HashSet<char>>(),
        ))
        .map(|sets| {
            *sets.0.intersection(&sets.1)
                .copied()
                .collect::<HashSet<char>>()
                .intersection(&sets.2)
                .next()
                .unwrap()
        })
        .map(|c| char_to_score(c))
        .sum();

    println!("{}", common);
}

fn char_to_score(c: char) -> u32 {
    return if c.is_lowercase() {
        (c as u32) - 96
    } else {
        (c as u32) - 38
    }
}