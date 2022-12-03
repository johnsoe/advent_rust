#[cfg(test)]
mod tests;

pub fn part_one(input: &Vec<Vec<u32>>) {
    let check_sum: u32 = input
        .iter()
        .map(|item| item.iter().max().unwrap() - item.iter().min().unwrap())
        .sum();

    println!("{}", check_sum);

}

pub fn part_two(input: &Vec<Vec<u32>>) {
    let check_sum: u32 = input
        .iter()
        .map(|row| {
            let multiplied = row.iter().fold(1, |sum, val| sum * val);
            let mut count = 2;
            let x = row.iter().map(|&x| multiplied * 1.0 / x)
        })
        .sum();
}