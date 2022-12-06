use itertools::Itertools;

#[cfg(test)]
mod tests;

pub fn part_one(instructions: Vec<(u32, u32, u32)>, mut stacks: Vec<String>) {
    instructions.iter().for_each(|(move_amt, from, to)| {
        for _x in 0..*move_amt {
            let pop = stacks[*from as usize].pop().unwrap();
            stacks[*to as usize].push(pop);
        }
    });
    print_top_of_stacks(stacks);
}

pub fn part_two(instructions: Vec<(u32, u32, u32)>, mut stacks: Vec<String>) {
    let mut temp_stack = String::new();
    instructions.iter().for_each(|(move_amt, from, to)| {
        for _x in 0..*move_amt {
            let pop = stacks[*from as usize].pop().unwrap();
            temp_stack.push(pop);
        }
        for _x in 0..*move_amt {
            let pop = temp_stack.pop().unwrap();
            stacks[*to as usize].push(pop);
        }
    });
    print_top_of_stacks(stacks);
}

fn print_top_of_stacks(stacks: Vec<String>) {
    println!("{}", stacks
        .iter()
        .map(|stack| stack.chars().last().unwrap())
        .join("")
    );
}