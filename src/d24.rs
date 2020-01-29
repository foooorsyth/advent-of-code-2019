use crate::shared::{in_bounds, neighbors, Point};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn part1() -> Result<i32> {
    let (mut img, w, h) = read("input/d24.txt")?;
    let mut set = HashSet::new();
    set.insert(img);
    loop {
        img = minute(img, w, h);
        if set.contains(&img) {
            return Ok(img);
        } else {
            set.insert(img);
        }
    }
}

fn minute(img: i32, w: usize, h: usize) -> i32 {
    let mut res = img;
    for pwr in 0..(w * h) {
        let near = neighbors(&Point::new((pwr % w) as i32, (pwr / w) as i32));
        let mut neighbor_bug_count = 0;
        for neighbor in near {
            let neighbor_pwr = (neighbor.y * (w as i32) + neighbor.x) as i32;
            if in_bounds(w, h, &neighbor) && ((img & 1 << neighbor_pwr) == (1 << neighbor_pwr)) {
                neighbor_bug_count += 1;
            }
        }
        if (img & 1 << (pwr as i32)) == (1 << (pwr as i32)) {
            if neighbor_bug_count != 1 {
                res &= !(1 << (pwr as i32));
            }
        } else {
            if neighbor_bug_count == 1 || neighbor_bug_count == 2 {
                res |= 1 << (pwr as i32);
            }
        }
    }
    res
}

pub fn read(input: &'static str) -> Result<(i32, usize, usize)> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut w = 0;
    let mut h = 0;
    let mut img: i32 = 0;
    let mut pwr = 0;
    for (y_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        w = line.len();
        h = y_index + 1;
        for (_, c) in line.chars().enumerate() {
            if c == '#' {
                img |= 1 << pwr;
                pwr += 1
            } else if c == '.' {
                pwr += 1;
            }
        }
    }
    Ok((img, w, h))
}
