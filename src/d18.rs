use crate::shared::{is_alpha, print_image, Point};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn part1() -> Result<()> {
    let (img, w, h, lut) = read("input/d18.txt")?;
    print_image(&img, w, h);
    Ok(())
}

fn read(input: &'static str) -> Result<(Vec<char>, usize, usize, HashMap<char, Point>)> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut w = 0;
    let mut h = 0;
    let mut img = Vec::<char>::new();
    let mut lut = HashMap::<char, Point>::new();
    for (y_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        w = line.len();
        h = y_index + 1;
        for (x_index, c) in line.chars().enumerate() {
            img.push(c);
            if is_alpha(c) || c == '@' {
                lut.insert(c, Point::new(x_index as i32, y_index as i32));
            }
        }
    }
    Ok((img, w, h, lut))
}
