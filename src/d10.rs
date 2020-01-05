use crate::shared::{distance_i32, get_pixel_at, set_pixel_at, Point, PointDist};
use std::collections::{BinaryHeap, HashMap};
use std::f32::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SPACE: u8 = 0;
const ASTEROID: u8 = 1;
const VISITED: u8 = 2;

pub fn part1() -> std::io::Result<i32> {
    let (mut img, w, h) = construct_image("input/d10.txt")?;
    let (_, visible_asteroids, _) = find_station(&mut img, &w, &h);
    return Ok(visible_asteroids);
}

pub fn part2() -> std::io::Result<i32> {
    let (mut img, w, h) = construct_image("input/d10.txt")?;
    let (_, _, mut lut) = find_station(&mut img, &w, &h);
    let pt = pew_pew(&mut lut, 200);
    return Ok(pt.x * 100 + pt.y);
}

pub fn pew_pew(lut: &mut HashMap<String, BinaryHeap<PointDist>>, target: i32) -> Point {
    // sort keys (angles)
    let mut f32_keys: Vec<f32> = lut.keys().map(|x| x.parse::<f32>().unwrap()).collect();
    f32_keys.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut index_of_q1 = 0;
    // could be more efficient here by acting like a ring buffer
    for (i, val) in f32_keys.iter().enumerate() {
        if *val <= PI / 2f32 {
            index_of_q1 = i;
        } else {
            break;
        }
    }
    return spin(lut, &f32_keys, index_of_q1, target);
}

fn spin(
    lut: &mut HashMap<String, BinaryHeap<PointDist>>,
    f32_keys: &Vec<f32>,
    index_of_q1: usize,
    target: i32,
) -> Point {
    let mut count = 0;
    loop {
        for i in (0..=index_of_q1).rev() {
            let fire_res = fire(lut, &f32_keys[i].to_string());
            if fire_res.is_some() {
                count += 1;
                if count == target {
                    return fire_res.unwrap();
                }
            }
        }
        for i in ((index_of_q1 + 1)..f32_keys.len()).rev() {
            let fire_res = fire(lut, &f32_keys[i].to_string());
            if fire_res.is_some() {
                count += 1;
                if count == target {
                    return fire_res.unwrap();
                }
            }
        }
    }
}

fn fire(lut: &mut HashMap<String, BinaryHeap<PointDist>>, key: &String) -> Option<Point> {
    let heap = lut.get_mut(key).unwrap();
    if heap.len() > 0 {
        let closest = heap.pop().unwrap();
        Some(closest.point)
    } else {
        None
    }
}

pub fn find_station(
    img: &mut Vec<u8>,
    w: &i32,
    h: &i32,
) -> (Point, i32, HashMap<String, BinaryHeap<PointDist>>) {
    let mut max_visible: i32 = 0;
    let mut station: Point = Point::origin();
    let mut lut: HashMap<String, BinaryHeap<PointDist>> = HashMap::new();
    for y in 0..*h {
        for x in 0..*w {
            let station_tmp = Point { x: x, y: y };
            let asteroid_lookup = asteroid_metadata(&mut img.clone(), *w, *h, &station_tmp);
            let visible = asteroid_lookup.keys().len() as i32;
            if visible > max_visible {
                max_visible = visible;
                station = station_tmp;
                lut = asteroid_lookup;
            }
        }
    }
    return (station, max_visible, lut);
}

fn asteroid_metadata(
    img: &mut Vec<u8>,
    w: i32,
    h: i32,
    p: &Point,
) -> HashMap<String, BinaryHeap<PointDist>> {
    let mut asteroid_lut: HashMap<String, BinaryHeap<PointDist>> = HashMap::new();
    flood_fill(img, w, h, p, p, &mut asteroid_lut);
    return asteroid_lut;
}

fn flood_fill(
    img: &mut Vec<u8>,
    w: i32,
    h: i32,
    center: &Point,
    current: &Point,
    asteroid_lut: &mut HashMap<String, BinaryHeap<PointDist>>,
) {
    if current.x < 0 || current.x > w - 1 || current.y < 0 || current.y > h - 1 {
        return;
    }

    let val = get_pixel_at(img, w, current);
    if val == VISITED {
        return;
    }
    set_pixel_at(img, w, current, VISITED);
    if center != current && val == ASTEROID {
        let angle = center.angle_to(&current).to_string();
        if !asteroid_lut.contains_key(&angle) {
            let mut heap = BinaryHeap::<PointDist>::new();
            heap.push(PointDist {
                point: current.clone(),
                dist: distance_i32(current, center),
            });
            asteroid_lut.insert(angle, heap);
        } else {
            asteroid_lut.get_mut(&angle).unwrap().push(PointDist {
                point: current.clone(),
                dist: distance_i32(current, center),
            });
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
        asteroid_lut,
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
        asteroid_lut,
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
        asteroid_lut,
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
        asteroid_lut,
    );
}

pub fn construct_image(data_file: &'static str) -> std::io::Result<(Vec<u8>, i32, i32)> {
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
