#[cfg(test)]
mod tests;

pub fn part_one(input: &str) {
    let mut x = 0;
    for c in input.chars() {
        match c {
            '(' => x += 1,
            ')' => x -= 1,
            _ => {},
        }
    }
    println!("{x}");
}

pub fn part_two(input: &str) {
    let mut x = 0;
    let mut index = 1;
    for c in input.chars() {
        match c {
            '(' => x += 1,
            ')' => x -= 1,
            _ => {},
        }
        if x < 0 {
            break;
        }
        index += 1;
    }
    println!("{index}");
}
