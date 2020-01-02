use crate::intcode::IntCodeCPU;
use std::collections::HashSet;

pub fn part1() -> std::io::Result<i32> {
    let mut mx = i32::min_value();
    let mut avail_phases = HashSet::<i32>::new();
    for phase in 0..=4 {
        avail_phases.insert(phase);
    }
    let data = IntCodeCPU::read_data_from_file("input/d7.txt")?;
    phase_permutations(data, &avail_phases, 0, 0, &mut mx);
    return Ok(mx);
}

fn phase_permutations(
    data: Vec<i32>,
    avail_phases: &HashSet<i32>,
    input_val: i32,
    depth: i32,
    max_output: &mut i32,
) {
    for phase in avail_phases {
        let mut cpu = IntCodeCPU::new();
        cpu.set_data(data.clone());
        cpu.enqueue_input(*phase);
        cpu.enqueue_input(input_val);
        cpu.execute();
        let res = cpu.last_output.unwrap();
        if depth < 4 {
            let mut fewer_phases = avail_phases.clone();
            fewer_phases.remove(&phase);
            phase_permutations(data.clone(), &fewer_phases, res, depth + 1, max_output);
        } else {
            if res > *max_output {
                *max_output = res;
            }
        }
    }
}
