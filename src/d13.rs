use crate::intcode::IntCodeCPU;
use crate::shared::{set_pixel_at, Point};

pub fn part1() -> std::io::Result<i32> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d13.txt")?;
    cpu.execute();
    let (img, _, _) = build_image(&cpu.output);
    return Ok(img.iter().filter(|&v| *v == 2).count() as i32);
}

fn build_image(output: &Vec<i64>) -> (Vec<u8>, usize, usize) {
    let mut x_min = i64::max_value();
    let mut x_max = i64::min_value();
    let mut y_min = i64::max_value();
    let mut y_max = i64::min_value();
    for i in (0..output.len()).step_by(3) {
        let x = output[i];
        let y = output[i + 1];
        if x < x_min {
            x_min = x;
        }
        if x > x_max {
            x_max = x;
        }
        if y < y_min {
            y_min = y;
        }
        if y > y_max {
            y_max = y;
        }
    }
    let w = (x_max - x_min) + 1;
    let h = (y_max - y_min) + 1;
    let mut img: Vec<u8> = vec![0; (w * h) as usize];
    for i in (0..output.len()).step_by(3) {
        let x = output[i];
        let y = output[i + 1];
        let v = output[i + 2];
        set_pixel_at(&mut img, w as i32, &Point::new(x as i32, y as i32), v as u8);
    }
    return (img, w as usize, h as usize);
}
