#[cfg(test)]
mod tests;

pub fn part_one(input: &[u32]) {

    let sum: u32 = input
        .iter().enumerate()
        .filter(|(index, &val)| val == input[(index + 1) % input.len()])
        .map(|(_index, &val)| val)
        .sum();
    println!("{}", sum);

}

pub fn part_two(input: &[u32]) {
    let len = input.len();
    let sum: u32 = input
        .iter().enumerate()
        .filter(|(index, &val)| val == input[(index + len/2) % len])
        .map(|(_index, &val)| val)
        .sum();
    println!("{}", sum);

}