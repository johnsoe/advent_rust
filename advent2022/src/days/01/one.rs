use std::collections::BTreeSet;

#[cfg(test)]
mod tests;


pub fn part_one(calories: Vec<Vec<u32>>) {
    let max: u32 = calories
        .iter()
        .map(|group| group.iter().sum())
        .max()
        .unwrap();
    println!("{}", max);
}

pub fn part_two(calories: Vec<Vec<u32>>) {
    let summed_max: u32 = calories
        .iter()
        .map(|group| group.iter().sum())
        .collect::<BTreeSet<u32>>()
        .iter()
        .rev()
        .take(3)
        .sum();

    println!("{}", summed_max);
}