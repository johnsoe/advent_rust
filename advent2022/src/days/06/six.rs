use std::collections::HashSet;

#[cfg(test)]
mod tests;

pub fn part_one(buffer: &str, message_length: usize) -> u32 {
    for (i, _item) in buffer.chars().enumerate() {
        let chunk = &buffer[i..i+message_length];
        let set: HashSet<char> = HashSet::from_iter(chunk.chars());
        if set.len() == chunk.len() {
            return (i + message_length) as u32;
        }
    }
    panic!("Shouldn't be here");
}