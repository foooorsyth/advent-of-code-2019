use crate::intcode;
use std::collections::HashSet;

pub fn part1() -> std::io::Result<i32> {
    let mut mx = i32::min_value();
    let mut avail_phases = HashSet::<i32>::new();
    for phase in 0..=4 {
        avail_phases.insert(phase);
    }
    phase_permutations("input/d7.txt", &avail_phases, 0, 0, &mut mx);
    return Ok(mx);
}

fn phase_permutations(
    data_file: &'static str,
    avail_phases: &HashSet<i32>,
    input_val: i32,
    depth: i32,
    max_output: &mut i32,
) {
    for phase in avail_phases {
        let mut input = Vec::<i32>::new();
        input.push(*phase);
        input.push(input_val);
        let res = intcode::execute_with_input(data_file, (false, 0), &input).unwrap();
        if depth < 4 {
            let mut fewer_phases = avail_phases.clone();
            fewer_phases.remove(&phase);
            phase_permutations(data_file, &fewer_phases, res, depth + 1, max_output)
        } else {
            if res > *max_output {
                *max_output = res;
            }
        }
    }
}
