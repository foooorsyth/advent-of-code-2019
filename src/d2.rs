use crate::intcode::IntCodeCPU;
use std::collections::HashMap;

const TARGET_PART2: i64 = 19690720;

pub fn part1() -> std::io::Result<i64> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d2.txt")?;
    cpu.set_data_at(1, 12);
    cpu.set_data_at(2, 2);
    cpu.execute();
    return Ok(cpu.get_data_at(0));
}

pub fn part2() -> std::io::Result<i64> {
    // brute force
    for n in 0..100 {
        for v in 0..100 {
            let mut ow = HashMap::new();
            ow.insert(1, n);
            ow.insert(2, v);
            let mut cpu = IntCodeCPU::new();
            cpu.read_data_file("input/d2.txt")?;
            cpu.set_data_at(1, n);
            cpu.set_data_at(2, v);
            cpu.execute();
            if cpu.get_data_at(0) == TARGET_PART2 {
                return Ok(100 * n + v);
            }
        }
    }
    panic!("wtf")
}
