use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let nums = std::fs::read_to_string("input.txt")?;
    let split = nums.split_whitespace();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for (i, item) in split.enumerate(){
        if i % 2 == 0{
            left.push(item.parse::<i32>().unwrap());
        }
        else{
            right.push(item.parse::<i32>().unwrap());
        }
    }

    left.sort();
    right.sort();

    let diff_sum = left.iter().
    zip(&right).
    map(|(l, r)| (l - r).abs()).sum::<i32>();
    println!("{}", diff_sum);
    

    let mut counts = std::collections::HashMap::new();
    for i in right{
        *counts.entry(i).or_insert(0) += 1;
    }

    let sim_score: i32 = left.iter().map(|num| {
        counts.get(num).copied().unwrap_or(0) * num}
        ).sum();

    println!("{}", sim_score);
    Ok(())
}
