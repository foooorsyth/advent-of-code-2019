use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn part1() -> Result<String> {
    let mut input: Vec<char> = read("input/d16.txt").unwrap().chars().collect();
    for _ in 0..100 {
        input = phase(&input);
    }
    let output_str: String = input.iter().collect();
    Ok((&output_str[..8]).to_owned())
}

pub fn part2() -> Result<String> {
    let base_input: Vec<i32> = read("input/d16.txt")
        .unwrap()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|i| i as i32)
        .collect();
    let offset = base_input
        .iter()
        .take(7)
        .fold(0, |acc, x| 10 * acc + *x as usize);
    let mut big_input: Vec<i32> = Vec::new();
    for _ in 0..10000 {
        big_input.append(&mut base_input.clone());
    }
    let mut segment: Vec<i32> = big_input[offset..big_input.len()].iter().cloned().collect();
    for _ in 0..100 {
        let mut new_segment: Vec<i32> = Vec::new();
        let mut last_sum = segment.iter().cloned().fold(0, |acc, x| acc + x);
        new_segment.push(last_sum % 10);
        for depth in 1..segment.len() {
            last_sum = last_sum - segment[depth - 1];
            new_segment.push(last_sum % 10);
        }
        segment.clear();
        segment.append(&mut new_segment);
    }
    let mut res = "".to_owned();
    for i in 0..8 {
        res.push_str(&segment[i].to_string());
    }
    Ok(res)
}

fn phase(input: &Vec<char>) -> Vec<char> {
    let mut res: Vec<char> = Vec::new();
    for depth in 1..=input.len() {
        let mut sign: i64 = 1;
        let mut sum: i64 = 0;
        // adds
        for i in ((depth - 1)..input.len()).step_by(depth * 2) {
            for w in 0..depth {
                let idx = i + w;
                if idx >= input.len() {
                    break;
                }
                let c = input[i + w];
                sum += (((c as u8) - 48) as i64) * sign;
            }
            sign = sign * -1;
        }
        res.push(((&sum.abs() % 10 + 48) as u8) as char);
    }
    res
}

fn read(data_file: &'static str) -> Result<String> {
    let mut f = File::open(data_file)?;
    let mut text = String::new();
    f.read_to_string(&mut text)?;
    Ok(text)
}
