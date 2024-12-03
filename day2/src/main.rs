use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut safe: i32 = 0;

    for report in reader.lines() {
        let nums: Vec<i32> = report?
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if perform_checks(&nums) {
            safe += 1
        }
    }
    println!("{}", safe);

    Ok(())
}

fn perform_checks(vec: &Vec<i32>) -> bool {
    let mut passed: Vec<bool> = Vec::new();
    for i in 0..vec.len() {
        let mut temp = vec.clone();
        temp.remove(i);
        passed.push(inc_or_dec(&temp) && bounds(&temp));
    }
    return passed.iter().any(|x| *x == true);
}

fn inc_or_dec(vec: &Vec<i32>) -> bool {
    let asc: bool = vec.iter().is_sorted();
    let desc: bool = vec.iter().rev().is_sorted();

    // if one of them is true then it is one of asc or desc
    return asc || desc;
}

fn bounds(vec: &Vec<i32>) -> bool {
    // checks whether bounds is bwtween 1 and 3
    let in_bounds: Vec<bool> = vec
        .windows(2)
        .map(|x| {
            let diff = (x[0] - x[1]).abs();
            if diff > 3 || diff < 1 {
                false
            } else {
                return true;
            }
        })
        .collect();
    return in_bounds.iter().all(|x| *x == true);
}
