use crate::intcode::{CPUState, IntCodeCPU};
use crate::shared::{print_image, set_pixel_at, visual_image, Point};
use std::collections::HashMap;
use std::io::Result;

pub fn part1() -> Result<i32> {
    return Ok(paint("input/d11.txt", 0).unwrap().keys().len() as i32);
}

pub fn part2(visualize: bool) -> Result<String> {
    let pixels = paint("input/d11.txt", 1).unwrap();
    let mut x_min = i32::max_value();
    let mut x_max = i32::min_value();
    let mut y_min = i32::max_value();
    let mut y_max = i32::min_value();
    for pt_str in pixels.keys() {
        let pt = Point::from_string((*pt_str).clone());
        if pt.x < x_min {
            x_min = pt.x;
        }
        if pt.x > x_max {
            x_max = pt.x;
        }
        if pt.y < y_min {
            y_min = pt.y;
        }
        if pt.y > y_max {
            y_max = pt.y;
        }
    }
    let w = (x_max - x_min) + 1;
    let h = (y_max - y_min) + 1;
    let mut img: Vec<u8> = vec![0; (w * h) as usize];
    for pt_str in pixels.keys() {
        // set values in image
        let mut relative_pt = Point::from_string((*pt_str).clone());
        relative_pt.x = relative_pt.x + x_min.abs();
        relative_pt.y = relative_pt.y + y_min.abs();
        set_pixel_at(&mut img, w, &relative_pt, pixels[pt_str]);
    }
    if visualize {
        print_image(&visual_image(&img), w as usize, h as usize);
    }
    return Ok(visual_image(&img).iter().collect());
}

fn paint(input_file: &'static str, initial_val: u8) -> Result<HashMap<String, u8>> {
    let mut cpu = IntCodeCPU::new();
    let mut pixels: HashMap<String, u8> = HashMap::new();
    let mut current = Point::origin();
    pixels.insert(current.to_string(), initial_val);
    let mut dir = 0;
    cpu.read_data_file(input_file)?;
    loop {
        cpu.execute();
        if cpu.output.len() == 2 {
            let color = cpu.output[0];
            let turn = cpu.output[1];
            let str_rep = current.to_string();
            // paint color
            *pixels.get_mut(&str_rep).unwrap() = color as u8;
            // change dir
            if turn == 1 {
                dir = (dir + 1) % 4;
            } else {
                dir = (dir - 1) % 4;
            }
            // move
            match dir {
                // up
                0 => {
                    current = Point {
                        x: current.x,
                        y: current.y - 1,
                    }
                }
                // right
                1 | -3 => {
                    current = Point {
                        x: current.x + 1,
                        y: current.y,
                    }
                }
                // down
                2 | -2 => {
                    current = Point {
                        x: current.x,
                        y: current.y + 1,
                    }
                }
                // left
                3 | -1 => {
                    current = Point {
                        x: current.x - 1,
                        y: current.y,
                    }
                }
                _ => panic!("wtf"),
            }
            cpu.output.clear();
        }
        match cpu.state {
            CPUState::WaitingForInput => {
                let mut current_px = 0;
                let str_rep = current.to_string();
                if pixels.contains_key(&str_rep) {
                    current_px = pixels[&str_rep];
                } else {
                    pixels.insert(str_rep, 0);
                }
                cpu.enqueue_input(current_px as i64);
            }
            CPUState::Halted => return Ok(pixels),
            _ => panic!("Illegal state"),
        }
    }
}
