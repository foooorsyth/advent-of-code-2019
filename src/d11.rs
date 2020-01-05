use crate::intcode::{CPUState, IntCodeCPU};
use crate::shared::Point;
use std::collections::HashMap;
use std::io::Result;

pub fn part1() -> Result<i32> {
    return paint("input/d11.txt");
}

fn paint(input_file: &'static str) -> Result<i32> {
    let mut cpu = IntCodeCPU::new();
    let mut pixels: HashMap<String, i8> = HashMap::new();
    let mut current = Point::origin();
    let mut dir = 0;
    cpu.read_data_file(input_file)?;
    loop {
        cpu.execute();
        if cpu.output.len() == 2 {
            let color = cpu.output[0];
            let turn = cpu.output[1];
            let str_rep = current.to_string();
            // paint color
            *pixels.get_mut(&str_rep).unwrap() = color as i8;
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
            CPUState::Halted => return Ok(pixels.keys().len() as i32),
            _ => panic!("Illegal state"),
        }
    }
}
