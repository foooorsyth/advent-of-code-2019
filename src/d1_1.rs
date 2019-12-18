use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

pub fn solve() -> std::io::Result<i32> {
    let file = File::open("input/d1_1.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        let int_val = line.parse::<i32>().unwrap();
        let fuel_required = fuel_required(&int_val);
        sum += fuel_required;
    }
    return Ok(sum)
}

fn fuel_required(module_mass: &i32) -> i32 {
    return module_mass / 3 - 2
}