use std::ops::Range;

#[cfg(test)]
mod tests;

pub fn part_one(sections: Vec<(Range<u32>, Range<u32>)>) {
    let overlapped_section_count = sections.iter()
        .filter(|ranges|{
            (ranges.0.start <= ranges.1.start && ranges.0.end >= ranges.1.end) ||
                (ranges.0.start >= ranges.1.start && ranges.0.end <= ranges.1.end)
        })
        .count();
    println!("{}", overlapped_section_count);
}

pub fn part_two(sections: Vec<(Range<u32>, Range<u32>)>) {
    let overlapped_section_count = sections.iter()
        .filter(|ranges|{
            (ranges.0.end >= ranges.1.start && ranges.0.start <= ranges.1.end) ||
                (ranges.1.end >= ranges.0.start && ranges.1.start <= ranges.0.end)
        })
        .count();
    println!("{}", overlapped_section_count);
}