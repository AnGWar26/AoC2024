use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let nums = std::fs::read_to_string("input.txt")?;
    let split = nums.split_whitespace();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

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

    let mut sum: i32 = 0;
    for i in 0..left.len(){
        sum = sum + (left[i] - right[i]).abs();
    }

    let mut counts: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for i in right{
        let _res = counts.entry(i).
        and_modify(|counter| *counter += 1).
        or_insert(1);
    }

    let mut sim_score: i32 = 0;
    for i in left{
        let res = counts.get(&i);
        match res{
            Some(freq) => sim_score += i * freq,
            _ => ()
        }
    }

    println!("{}", sim_score);
    Ok(())
}
