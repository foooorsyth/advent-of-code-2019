use crate::ds::Point;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SPACE: u8 = 0;
const ASTEROID: u8 = 1;
const VISITED: u8 = 2;

pub fn part1() -> std::io::Result<i32> {
    let (img, w, h) = construct_image("input/d10.txt")?;
    let mut mx: i32 = 0;
    for y in 0..h {
        for x in 0..w {
            let asteroid_count = count_asteroids(&mut img.clone(), w, h, &Point { x: x, y: y });
            if asteroid_count > mx {
                mx = asteroid_count;
            }
        }
    }
    return Ok(mx);
}

fn count_asteroids(img: &mut Vec<u8>, w: i32, h: i32, p: &Point) -> i32 {
    let mut angle_cache: Vec<f32> = Vec::new();
    flood_fill(img, w, h, p, p, &mut angle_cache);
    return angle_cache.len() as i32;
}

fn flood_fill(
    img: &mut Vec<u8>,
    w: i32,
    h: i32,
    center: &Point,
    current: &Point,
    angles: &mut Vec<f32>,
) {
    if current.x < 0 || current.x > w - 1 || current.y < 0 || current.y > h - 1 {
        return;
    }

    let val = get_pixel_at(img, w, current);
    if val == VISITED {
        return;
    } else {
        set_pixel_at(img, w, current, VISITED);
        if center != current && val == ASTEROID {
            let angle = center.angle_to(&current);
            if !angles.contains(&angle) {
                angles.push(angle);
            }
        }
        flood_fill(
            img,
            w,
            h,
            center,
            &Point {
                x: current.x - 1,
                y: current.y,
            },
            angles,
        );
        flood_fill(
            img,
            w,
            h,
            center,
            &Point {
                x: current.x + 1,
                y: current.y,
            },
            angles,
        );
        flood_fill(
            img,
            w,
            h,
            center,
            &Point {
                x: current.x,
                y: current.y + 1,
            },
            angles,
        );
        flood_fill(
            img,
            w,
            h,
            center,
            &Point {
                x: current.x,
                y: current.y - 1,
            },
            angles,
        );
    }
}

fn get_pixel_at(img: &Vec<u8>, w: i32, p: &Point) -> u8 {
    return img[(w * p.y + p.x) as usize];
}

fn set_pixel_at(img: &mut Vec<u8>, w: i32, p: &Point, val: u8) {
    img[(w * p.y + p.x) as usize] = val;
}

fn construct_image(data_file: &'static str) -> std::io::Result<(Vec<u8>, i32, i32)> {
    let file = File::open(data_file)?;
    let reader = BufReader::new(file);
    let mut w = 0;
    let mut h = 0;
    let mut img: Vec<u8> = Vec::new();
    for (h_index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        w = line.len();
        for c in line.chars() {
            match c {
                '.' => img.push(SPACE),
                '#' => img.push(ASTEROID),
                _ => panic!("Illegal char"),
            }
        }
        h = h_index + 1;
    }
    return Ok((img, w as i32, h as i32));
}
