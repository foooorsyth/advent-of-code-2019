use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub const fn origin() -> Point {
        return Point { x: 0, y: 0 }
    }
}

pub fn part1() -> std::io::Result<i32> {
    let file = File::open("input/d3.txt")?;
    let reader = BufReader::new(file);
    let mut cache: HashSet<String> = HashSet::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        let instructions: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
        let closest = walk(&instructions, &mut cache, &(index as i32));
        if index == 1 {
            return Ok(closest)
        }
    }
    panic!("wtf")
}

fn point_to_string(point: &Point) -> String {
    return format!("{}{}{}", point.x.to_string(), ",", point.y.to_string())
}

fn walk(instructions: &Vec<String>, cache: &mut HashSet<String>, pass: &i32) -> i32 {
    let mut last = Point { x: 0, y: 0 };
    let mut closest_distance: i32 = i32::max_value();
    for instruction in instructions {
        let (dir, seg_len_str) = instruction.split_at(1);
        let seg_len = seg_len_str.parse::<i32>().unwrap();
        let step: (i32, i32);
        match dir {
            "U" => {
                step = (0, -1);
            }
            "D" => {
                step = (0, 1);
            }
            "L" => {
                step = (-1, 0);
            }
            "R" => {
                step = (1, 0);
            }
            &_ => { panic!("wtf") }
        }
        for _ in 1..=seg_len {
            last = Point { x: last.x + step.0, y: last.y + step.1 };
            if *pass == 0 {
                cache.insert(point_to_string(&last.clone()));
            } else {
                if cache.contains(&point_to_string(&last.clone())) {
                    let dist = manhattan_distance(&Point::origin(), &last);
                    if dist < closest_distance {
                        closest_distance = dist;
                    }
                }
            }
        }
    }
    return closest_distance
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    return  (a.x - b.x).abs() + (a.y - b.y).abs()
}