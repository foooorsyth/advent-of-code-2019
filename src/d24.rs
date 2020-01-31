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

pub fn part2() -> Result<i32> {
    layered(200)
}

fn layered(minutes: usize) -> Result<i32> {
    let (base, w, h) = read("input/d24.txt")?;
    // 200 (below) + 200 (above) + 1 (zero-level) + 2 (buffer to avoid out of bounds)
    let mut layers = vec![0; minutes * 2 + 3];
    layers[(minutes * 2 + 3) / 2] = base;
    for _ in 0..minutes {
        layers = minute_layered(&mut layers, w, h);
    }
    let res = layers.iter().fold(0, |acc, img| -> i32 {
        let mut ones = 0;
        for pwr in 0..(w * h) {
            if (img & 1 << (pwr as i32)) == (1 << (pwr as i32)) {
                ones += 1;
            }
        }
        acc + ones
    });
    Ok(res)
}

fn minute_layered(imgs: &mut Vec<i32>, w: usize, h: usize) -> Vec<i32> {
    let mut res = imgs.clone();
    for img_idx in 1..(imgs.len() - 1) {
        for pwr in 0..(w * h) {
            if pwr == (w * h) / 2 {
                continue;
            }
            let near = neighbors_layered(
                img_idx,
                &Point::new((pwr % w) as i32, (pwr / w) as i32),
                w,
                h,
            );
            let mut neighbor_bug_count = 0;
            for neighbor in near {
                let neighbor_pwr = (neighbor.0.y * (w as i32) + neighbor.0.x) as i32;
                if (imgs[neighbor.1] & 1 << neighbor_pwr) == (1 << neighbor_pwr) {
                    neighbor_bug_count += 1;
                }
            }
            if (imgs[img_idx] & 1 << (pwr as i32)) == (1 << (pwr as i32)) {
                if neighbor_bug_count != 1 {
                    res[img_idx] &= !(1 << (pwr as i32));
                }
            } else {
                if neighbor_bug_count == 1 || neighbor_bug_count == 2 {
                    res[img_idx] |= 1 << (pwr as i32);
                }
            }
        }
    }
    res
}

// all returned will be in bounds
fn neighbors_layered(img_idx: usize, p: &Point, w: usize, h: usize) -> Vec<(Point, usize)> {
    let mut res = Vec::new();
    let near = neighbors(&p);
    for neighbor in near {
        if in_bounds(w, h, &neighbor) {
            res.push((neighbor.clone(), img_idx));
        }
    }
    // outside to container

    // left
    if p.x == 0 {
        res.push((Point::new(1, 2), img_idx - 1));
    }
    // top
    if p.y == 0 {
        res.push((Point::new(2, 1), img_idx - 1));
    }
    // right
    if p.x == (w - 1) as i32 {
        res.push((Point::new(3, 2), img_idx - 1));
    }
    // bottom
    if p.y == (h - 1) as i32 {
        res.push((Point::new(2, 3), img_idx - 1));
    }

    // inside to contained

    // left
    if p.x == 1 && p.y == 2 {
        for y in 0..h {
            res.push((Point::new(0, y as i32), img_idx + 1))
        }
    }

    // top
    if p.x == 2 && p.y == 1 {
        for x in 0..w {
            res.push((Point::new(x as i32, 0), img_idx + 1))
        }
    }

    // right
    if p.x == 3 && p.y == 2 {
        for y in 0..h {
            res.push((Point::new((w - 1) as i32, y as i32), img_idx + 1))
        }
    }

    // bottom
    if p.x == 2 && p.y == 3 {
        for x in 0..w {
            res.push((Point::new(x as i32, (h - 1) as i32), img_idx + 1))
        }
    }

    res
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
