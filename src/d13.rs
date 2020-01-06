use crate::intcode::{CPUState, IntCodeCPU};
use crate::shared::{print_image, set_pixel_at, visual_image, Point};

pub fn part1() -> std::io::Result<i32> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d13.txt")?;
    cpu.execute();
    let (img, _, _, _, _) = build_image(&cpu.output, &mut 0);
    return Ok(img.iter().filter(|&v| *v == 2).count() as i32);
}

pub fn part2() -> std::io::Result<i64> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d13.txt")?;
    cpu.set_data_at(0, 2);
    let mut score: i64 = 0;
    loop {
        cpu.execute();
        let (img, w, h, ball_x, paddle_x) = build_image(&cpu.output, &mut score);
        println!("Score: {}", score);
        print_image(&visual_image(&img), w as usize, h as usize);
        match cpu.state {
            CPUState::WaitingForInput => {
                // Aimbot!
                let diff = (ball_x as i32) - (paddle_x as i32);
                if diff > 0 {
                    cpu.enqueue_input(1);
                } else if diff < 0 {
                    cpu.enqueue_input(-1);
                } else {
                    cpu.enqueue_input(0);
                }
            }
            CPUState::Halted => {
                return Ok(score);
            }
            _ => {
                panic!("Illegal state");
            }
        }
    }
}

fn build_image(output: &Vec<i64>, last_score: &mut i64) -> (Vec<u8>, usize, usize, usize, usize) {
    let mut x_max = i64::min_value();
    let mut y_max = i64::min_value();
    for i in (0..output.len()).step_by(3) {
        let x = output[i];
        let y = output[i + 1];
        if x > x_max {
            x_max = x;
        }
        if y > y_max {
            y_max = y;
        }
    }
    let w = x_max + 1;
    let h = y_max + 1;
    let mut img: Vec<u8> = vec![0; (w * h) as usize];
    let mut ball_x = 0;
    let mut paddle_x = 0;
    for i in (0..output.len()).step_by(3) {
        let x = output[i];
        let y = output[i + 1];
        let v = output[i + 2];
        if x == -1 && y == 0 {
            *last_score = v;
        } else {
            set_pixel_at(&mut img, w as i32, &Point::new(x as i32, y as i32), v as u8);
            match v {
                3 => paddle_x = x,
                4 => {
                    ball_x = x;
                }
                _ => {}
            }
        }
    }
    return (
        img,
        w as usize,
        h as usize,
        ball_x as usize,
        paddle_x as usize,
    );
}
