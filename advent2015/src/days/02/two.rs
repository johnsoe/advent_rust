use anyhow::{ Result };

#[cfg(test)]
mod tests;

pub fn part_one(dimens: Vec<Vec<i32>>) -> Result<i32> {
    let total = dimens.iter()
        .map( |nums| {
            let a: i32 = nums[0] * nums[1];
            let b: i32 = nums[0] * nums[2];
            let c: i32 = nums[1] * nums[2];
            
            (a + b + c) * 2 + a.min(b).min(c)
        })
        .sum();

    Ok(total)
}

pub fn part_two(dimens: Vec<Vec<i32>>) -> Result<i32> {
    let total = dimens.iter()
        .map( |nums| {
            
            let bow: i32 = nums.iter().product();
            let max: i32 = nums[0].max(nums[1]).max(nums[2]);
            let ribbon: i32 = (nums.iter().sum::<i32>() - max) * 2;
            
            bow + ribbon
        })
        .sum();

    Ok(total)
}
