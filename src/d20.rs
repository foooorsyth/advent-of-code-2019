use crate::shared::{is_alpha, Point};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn part1() -> Result<usize> {
    let (mut img, w, _h, portals, start, end) = read("input/d20.txt")?;
    let mut q = VecDeque::<(Point, usize)>::new();
    let near = neighbors(&start);
    for p in near {
        if img[(p.y as usize) * w + (p.x as usize)] == '.' {
            q.push_back((p.clone(), 1));
        }
    }
    img[(start.y as usize) * w + (start.x as usize)] = '#'; // mark visited
    loop {
        if q.len() == 0 {
            panic!("couldn't find end");
        }
        let current = q.pop_front().unwrap();
        img[(current.0.y as usize) * w + (current.0.x as usize)] = '#'; // mark visited
        let near = neighbors(&current.0);
        for p in near {
            if p == end {
                return Ok(current.1 + 1);
            }
            let c = img[(p.y as usize) * w + (p.x as usize)];
            if c == '.' {
                q.push_back((p.clone(), current.1 + 1));
            } else if is_alpha(c) {
                let dest = portals[&current.0.to_string()];
                q.push_back((dest.clone(), current.1 + 1));
            }
        }
    }
}

pub fn part2() -> Result<usize> {
    let (img, w, h, portals, start, end) = read("input/d20.txt")?;
    let mut q = VecDeque::<(
        Point,
        /* step depth */ usize,
        /*maze level*/ usize,
    )>::new();
    let mut levels = Vec::<Vec<char>>::new();
    levels.push(img.clone());
    let near = neighbors(&start);
    for p in near {
        if levels[0][(p.y as usize) * w + (p.x as usize)] == '.' {
            q.push_back((p.clone(), 1, 0));
        }
    }
    levels[0][(start.y as usize) * w + (start.x as usize)] = '#'; // mark visited
    loop {
        if q.len() == 0 {
            panic!("couldn't find end");
        }
        let current = q.pop_front().unwrap();
        let current_pt = current.0;
        let step_depth = current.1;
        let maze_level = current.2;
        if levels.len() == maze_level {
            levels.push(img.clone());
        }
        levels[maze_level][(current_pt.y as usize) * w + (current_pt.x as usize)] = '#'; // mark visited
        let near = neighbors(&current_pt);
        for p in near {
            if p == end && maze_level == 0 {
                return Ok(step_depth + 1);
            }
            let c = levels[maze_level][(p.y as usize) * w + (p.x as usize)];
            if c == '.' {
                q.push_back((p.clone(), step_depth + 1, maze_level));
            } else if is_alpha(c) {
                if current_pt == start || (current_pt == end && maze_level != 0) {
                    continue;
                }
                let dest = portals[&current_pt.to_string()];
                if maze_level == 0 {
                    if is_inner(&current_pt, w, h) {
                        q.push_back((dest.clone(), step_depth + 1, maze_level + 1));
                    }
                } else {
                    if is_inner(&current_pt, w, h) {
                        q.push_back((dest.clone(), current.1 + 1, maze_level + 1));
                    } else {
                        q.push_back((dest.clone(), current.1 + 1, maze_level - 1));
                    }
                }
            }
        }
    }
}

fn is_inner(p: &Point, w: usize, h: usize) -> bool {
    p.x != 2 && p.y != 2 && p.x != ((w - 3) as i32) && p.y != ((h - 3) as i32)
}

fn neighbors(p: &Point) -> Vec<Point> {
    let mut res = Vec::new();
    // n
    res.push(Point::new(p.x, p.y - 1));
    // e
    res.push(Point::new(p.x + 1, p.y));
    // s
    res.push(Point::new(p.x, p.y + 1));
    // w
    res.push(Point::new(p.x - 1, p.y));
    res
}

fn read(
    input: &'static str,
) -> Result<(
    Vec<char>,
    usize,
    usize,
    HashMap<String, Point>,
    Point,
    Point,
)> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    let mut w = 0;
    let mut h = 0;
    let mut l = 0;
    let mut t = 0;
    let mut r = 0;
    let mut b = 0;
    let mut img = Vec::<char>::new();
    // position to position
    let mut portals = HashMap::<String, Point>::new();
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        w = line.len();
        h = y + 1;
        let mut contains_wall = false;
        for (x, c) in line.chars().enumerate() {
            img.push(c);
            if c == '#' {
                contains_wall = true;
            } else if c == ' ' && contains_wall && x >= 2 && x < w - 2 && y >= 2 {
                if t == 0 {
                    t = y;
                } else {
                    b = y;
                }
                if l == 0 {
                    l = x;
                } else {
                    r = x;
                }
            }
        }
    }
    // label to position
    let mut portals_inverse = HashMap::<String, Point>::new();
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);
    for y in 0..h {
        for x in 0..w {
            let c = img[y * w + x];
            if is_alpha(c) {
                // left outer or right inner, read right
                if x == 0 || x == (r - 1) {
                    let mut label = String::new();
                    label.push(c);
                    label.push(img[y * w + x + 1]);
                    let p = Point::new((x + 2) as i32, y as i32);
                    if label == "AA" {
                        start = p.clone();
                    }
                    if label == "ZZ" {
                        end = p.clone();
                    }
                    if !portals_inverse.contains_key(&label) {
                        portals_inverse.insert(label, p);
                    } else {
                        let dest = portals_inverse[&label];
                        portals.insert(p.to_string(), dest.clone());
                        portals.insert(dest.to_string(), p.clone());
                    }
                }
                // top outer or bottom inner, read down
                else if y == 0 || (y == (b - 1) && x >= 2 && x < w - 2) {
                    let mut label = String::new();
                    label.push(c);
                    label.push(img[(y + 1) * w + x]);
                    let p = Point::new(x as i32, (y + 2) as i32);
                    if label == "AA" {
                        start = p.clone();
                    }
                    if label == "ZZ" {
                        end = p.clone();
                    }
                    if !portals_inverse.contains_key(&label) {
                        portals_inverse.insert(label, p);
                    } else {
                        let dest = portals_inverse[&label];
                        portals.insert(p.to_string(), dest.clone());
                        portals.insert(dest.to_string(), p.clone());
                    }
                }
                // right outer or left inner, read right, point left
                else if x == w - 2 || x == l {
                    let mut label = String::new();
                    label.push(c);
                    label.push(img[y * w + x + 1]);
                    let p = Point::new((x - 1) as i32, y as i32);
                    if label == "AA" {
                        start = p.clone();
                    }
                    if label == "ZZ" {
                        end = p.clone();
                    }
                    if !portals_inverse.contains_key(&label) {
                        portals_inverse.insert(label, p);
                    } else {
                        let dest = portals_inverse[&label];
                        portals.insert(p.to_string(), dest.clone());
                        portals.insert(dest.to_string(), p.clone());
                    }
                }
                // bottom outer or top inner, read down, point up
                else if y == h - 2 || (y == t && x >= 2 && x < w - 2) {
                    let mut label = String::new();
                    label.push(c);
                    label.push(img[(y + 1) * w + x]);
                    let p = Point::new(x as i32, (y - 1) as i32);
                    if label == "AA" {
                        start = p.clone();
                    }
                    if label == "ZZ" {
                        end = p.clone();
                    }
                    if !portals_inverse.contains_key(&label) {
                        portals_inverse.insert(label, p);
                    } else {
                        let dest = portals_inverse[&label];
                        portals.insert(p.to_string(), dest.clone());
                        portals.insert(dest.to_string(), p.clone());
                    }
                }
            }
        }
    }
    Ok((img, w, h, portals, start, end))
}
