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
