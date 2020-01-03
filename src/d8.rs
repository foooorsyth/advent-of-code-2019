use std::fs::File;
use std::io::prelude::*;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

pub fn part1() -> std::io::Result<i32> {
    let image_data = read_data("input/d8.txt")?;
    let layer_count = image_data.len() / (WIDTH * HEIGHT);
    let mut fewest_zeros = (i32::max_value(), 0, 0);
    for layer in 0..layer_count {
        let (zeros, ones, twos) = count_values(&image_data, layer, WIDTH, HEIGHT);
        if zeros < fewest_zeros.0 {
            fewest_zeros = (zeros, ones, twos);
        }
    }
    return Ok(fewest_zeros.1 * fewest_zeros.2);
}

pub fn part2() -> std::io::Result<()> {
    let image_data = read_data("input/d8.txt")?;
    let constructed_image = construct_image(&image_data, WIDTH, HEIGHT);
    print_image(&constructed_image, WIDTH, HEIGHT);
    return Ok(());
}

fn construct_image(image_data: &Vec<u32>, w: usize, h: usize) -> Vec<u32> {
    let mut res = Vec::<u32>::new();
    let layer_count = image_data.len() / (WIDTH * HEIGHT);
    for y in 0..h {
        for x in 0..w {
            for layer in 0..layer_count {
                let pixel_val = image_data[(w * h * layer) + (w * y + x)];
                if pixel_val != 2 {
                    // if not transparent
                    res.push(pixel_val);
                    break;
                }
            }
        }
    }
    return res;
}

fn print_image(image_data: &Vec<u32>, w: usize, h: usize) {
    for y in 0..h {
        let row_start = w * y;
        let row_end = row_start + w;
        println!("{:?}", &image_data[row_start..row_end])
    }
}

fn count_values(image_data: &Vec<u32>, layer: usize, w: usize, h: usize) -> (i32, i32, i32) {
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
