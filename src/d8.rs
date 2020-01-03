use std::fs::File;
use std::io::prelude::*;

pub fn part1() -> std::io::Result<i32> {
    let image_data = read_data("input/d8.txt")?;
    let w = 25;
    let h = 6;
    let layer_count = image_data.len() / (w * h);
    let mut fewest_zeros = (i32::max_value(), 0, 0);
    for layer in 0..layer_count {
        let (zeros, ones, twos) = read_layer(&image_data, layer, w, h);
        if zeros < fewest_zeros.0 {
            fewest_zeros = (zeros, ones, twos);
        }
    }
    return Ok(fewest_zeros.1 * fewest_zeros.2);
}

fn read_layer(image_data: &Vec<u32>, layer: usize, w: usize, h: usize) -> (i32, i32, i32) {
    let mut zero_count = 0;
    let mut one_count = 0;
    let mut two_count = 0;
    for y in 0..h {
        for x in 0..w {
            let pixel_val = image_data[(w * h * layer) + (w * y + x)];
            match pixel_val {
                0 => zero_count += 1,
                1 => one_count += 1,
                2 => two_count += 1,
                _ => {}
            }
        }
    }
    return (zero_count, one_count, two_count);
}

fn read_data(data_file: &'static str) -> std::io::Result<Vec<u32>> {
    let mut f = File::open(data_file)?;
    let mut text = String::new();
    f.read_to_string(&mut text)?;
    let int_vec: Vec<u32> = text.chars().map(|c| c.to_digit(10).unwrap()).collect();
    return Ok(int_vec);
}
