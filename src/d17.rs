use crate::intcode::IntCodeCPU;
use crate::shared::{ascii_image, Point};

pub fn part1() -> std::io::Result<i64> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d17.txt")?;
    cpu.execute();
    let mut img = cpu.output.clone();
    println!("{}", ascii_image(&img));
    let w: usize = img.iter().position(|&x| x == 10).unwrap();
    img.retain(|&x| x != 10);
    let h: usize = img.len() / w;
    let intersecs = intersections(&img, w, h);
    let res: i64 = intersecs
        .iter()
        .fold(0, |acc, x| acc + (x.x as i64) * (x.y as i64));
    return Ok(res);
}

fn intersections(img: &Vec<i64>, w: usize, h: usize) -> Vec<Point> {
    let mut res = Vec::new();
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            if img[w * y + x] == 35
                && img[w * y + (x - 1)] == 35
                && img[w * y + (x + 1)] == 35
                && img[w * (y - 1) + x] == 35
                && img[w * (y + 1) + x] == 35
            {
                res.push(Point::new(x as i32, y as i32))
            }
        }
    }
    res
}
