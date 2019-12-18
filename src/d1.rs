use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> std::io::Result<i32> {
    return common(fuel_required);
}

pub fn part2() -> std::io::Result<i32> {
    return common(recursive_fuel_required);
}

fn fuel_required(module_mass: &i32) -> i32 {
    return module_mass / 3 - 2
}

fn recursive_fuel_required(module_mass: &i32) -> i32 {
    let current = module_mass / 3 - 2;
    if current < 0 {
        return 0;
    }
    return current + recursive_fuel_required(&current);
}

fn common(f: impl Fn(&i32) -> i32) -> std::io::Result<i32> {
    let file = File::open("input/d1.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        let int_val = line.parse::<i32>().unwrap();
        let fuel_required = f(&int_val);
        sum += fuel_required;
    }
    return Ok(sum)
}
