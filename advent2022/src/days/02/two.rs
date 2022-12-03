#[cfg(test)]
mod tests;


pub fn part_one(rps: Vec<(&str, &str)>) {
    let score: u32 = rps.iter()
        .map(|&game| calculate_game_score(game))
        .sum();

    println!("{}", score);
}

fn calculate_game_score(game: (&str, &str)) -> u32 {
    return match game {
        ("A", "X") => 4,
        ("A", "Y") => 8,
        ("A", "Z") => 3,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 7,
        ("C", "Y") => 2,
        ("C", "Z") => 6,
        _ => 0
    };
}

pub fn part_two(rps: Vec<(&str, &str)>) {
    let score: u32 = rps.iter()
        .map(|&game| calculate_proper_game_score(game))
        .sum();

    println!("{}", score);
}

fn calculate_proper_game_score(game: (&str, &str)) -> u32 {
    return match game {
        ("A", "X") => 3,
        ("A", "Y") => 4,
        ("A", "Z") => 8,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 2,
        ("C", "Y") => 6,
        ("C", "Z") => 7,
        _ => 0
    };
}