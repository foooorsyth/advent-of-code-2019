use crate::intcode::{CPUState, IntCodeCPU};
use std::collections::HashSet;

pub fn part1() -> std::io::Result<i32> {
    return phase_permutations(0, 4);
}

pub fn part2() -> std::io::Result<i32> {
    return phase_permutations(5, 9);
}

fn phase_permutations(low_inclusive: i32, hi_inclusive: i32) -> std::io::Result<i32> {
    let mut avail_phases = HashSet::<i32>::new();
    for phase in low_inclusive..=hi_inclusive {
        avail_phases.insert(phase);
    }
    let data = IntCodeCPU::read_data_from_file("input/d7.txt")?;
    let mut cpus = Vec::<IntCodeCPU>::new();
    for _ in 0..=4 {
        let cpu = IntCodeCPU::new();
        cpus.push(cpu);
    }
    let mut max_output = i32::min_value();
    phase_permutations_aux(
        &mut cpus,
        &data,
        &avail_phases,
        0,
        &mut vec![0; 5],
        &mut max_output,
    );
    return Ok(max_output);
}

#[allow(dead_code)]
fn phase_permutations_aux(
    cpus: &mut Vec<IntCodeCPU>,
    data: &Vec<i32>,
    avail_phases: &HashSet<i32>,
    depth: usize,
    curr_phase_vec: &mut Vec<i32>,
    max_output: &mut i32,
) {
    let cpu_count = cpus.len();
    for phase in avail_phases {
        curr_phase_vec[depth] = *phase;
        if depth < cpu_count - 1 {
            let mut fewer_phases = avail_phases.clone();
            fewer_phases.remove(&phase);
            phase_permutations_aux(
                cpus,
                data,
                &fewer_phases,
                depth + 1,
                curr_phase_vec,
                max_output,
            );
        } else {
            let phases = curr_phase_vec.to_vec();
            let mut pass = 0;
            let mut last_output = 0;
            'feedback: loop {
                for i in 0..cpu_count {
                    if pass == 0 {
                        cpus[i].reset(data.clone());
                        // Provide phase as input on first pass only
                        cpus[i].enqueue_input(phases[i]);
                    }
                    cpus[i].enqueue_input(last_output);
                    cpus[i].execute();
                    last_output = cpus[i].last_output.unwrap();
                    if i == cpu_count - 1 {
                        // At end of pass, check if last CPU is halted
                        if cpus[i].state == CPUState::Halted {
                            if last_output > *max_output {
                                *max_output = last_output;
                            }
                            break 'feedback;
                        }
                    }
                }
                pass += 1;
            }
        }
    }
}
