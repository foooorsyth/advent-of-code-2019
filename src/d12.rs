use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Body {
    p: [i32; 3],
    v: [i32; 3],
}

impl Body {
    fn new() -> Body {
        return Body {
            p: [0; 3],
            v: [0; 3],
        };
    }
}

pub fn part1() -> Result<i32> {
    let mut bodies = read("input/d12.txt")?;
    main_loop(&mut bodies, 1000);
    let mut energy_sum = 0;
    for body in bodies {
        let en = energy(&body);
        energy_sum += en.0 * en.1;
    }
    return Ok(energy_sum);
}

fn main_loop(bodies: &mut Vec<Body>, iterations: usize) {
    for _ in 0..iterations {
        apply_grav(bodies);
        apply_vel(bodies);
    }
}

fn apply_grav(bodies: &mut Vec<Body>) {
    for i in 0..bodies.len() {
        for j in i..bodies.len() {
            for c in 0..3 {
                grav_inner(bodies, i, j, c);
            }
        }
    }
}

fn grav_inner(bodies: &mut Vec<Body>, i: usize, j: usize, c: usize) {
    if bodies[i].p[c] > bodies[j].p[c] {
        bodies[i].v[c] -= 1;
        bodies[j].v[c] += 1;
    } else if bodies[i].p[c] < bodies[j].p[c] {
        bodies[i].v[c] += 1;
        bodies[j].v[c] -= 1;
    }
}

fn apply_vel(bodies: &mut Vec<Body>) {
    for body in bodies {
        for c in 0..3 {
            body.p[c] += body.v[c];
        }
    }
}

fn read(data_file: &'static str) -> Result<Vec<Body>> {
    let file = File::open(data_file)?;
    let reader = BufReader::new(file);
    let mut bodies = Vec::<Body>::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        let xyz: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
        let mut body = Body::new();
        for (index, coord) in xyz.iter().enumerate() {
            let eq = coord.find('=').unwrap();
            let res: i32;
            if index == 2 {
                res = coord[(eq + 1)..(coord.len() - 1)].parse::<i32>().unwrap();
            } else {
                res = coord[(eq + 1)..].parse::<i32>().unwrap();
            }
            body.p[index] = res;
        }
        bodies.push(body);
    }
    return Ok(bodies);
}

fn energy(body: &Body) -> (i32, i32) {
    (
        body.p[0].abs() + body.p[1].abs() + body.p[2].abs(),
        body.v[0].abs() + body.v[1].abs() + body.v[2].abs(),
    )
}
