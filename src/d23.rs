use crate::intcode::IntCodeCPU;

pub fn part1() -> std::io::Result<i64> {
    let mut base = IntCodeCPU::new();
    base.read_data_file("input/d23.txt")?;
    let mut comps: Vec<IntCodeCPU> = Vec::new();
    for i in 0..50 {
        comps.push(base.snapshot());
        comps[i].enqueue_input(i as i64);
    }
    loop {
        for cpu_idx in 0..50 {
            comps[cpu_idx].execute();
            if comps[cpu_idx].has_output() {
                for out_idx in (0..comps[cpu_idx].output.len()).step_by(3) {
                    let dest_idx = comps[cpu_idx].output[out_idx];
                    let x = comps[cpu_idx].output[out_idx + 1];
                    let y = comps[cpu_idx].output[out_idx + 2];
                    if dest_idx == 255 {
                        return Ok(y);
                    }
                    comps[dest_idx as usize].enqueue_input(x);
                    comps[dest_idx as usize].enqueue_input(y);
                }
                comps[cpu_idx].output.clear();
            } else {
                comps[cpu_idx].enqueue_input(-1);
            }
        }
    }
}

pub fn part2() -> std::io::Result<i64> {
    let mut base = IntCodeCPU::new();
    base.read_data_file("input/d23.txt")?;
    let mut comps: Vec<IntCodeCPU> = Vec::new();
    for i in 0..50 {
        comps.push(base.snapshot());
        comps[i].enqueue_input(i as i64);
    }
    let mut nat = (0, 0);
    let mut last_y = 0;
    let mut sent_from_nat = false;
    loop {
        let mut idle = true;
        for cpu_idx in 0..50 {
            comps[cpu_idx].execute();
            if comps[cpu_idx].has_output() {
                idle = false;
                for out_idx in (0..comps[cpu_idx].output.len()).step_by(3) {
                    let dest_idx = comps[cpu_idx].output[out_idx];
                    let x = comps[cpu_idx].output[out_idx + 1];
                    let y = comps[cpu_idx].output[out_idx + 2];
                    if dest_idx == 255 {
                        nat.0 = x;
                        nat.1 = y;
                    } else {
                        comps[dest_idx as usize].enqueue_input(x);
                        comps[dest_idx as usize].enqueue_input(y);
                    }
                }
                comps[cpu_idx].output.clear();
            } else {
                comps[cpu_idx].enqueue_input(-1);
            }
        }
        if idle {
            if sent_from_nat && nat.1 == last_y {
                return Ok(nat.1);
            }
            comps[0].enqueue_input(nat.0);
            comps[0].enqueue_input(nat.1);
            sent_from_nat = true;
            last_y = nat.1;
        }
    }
}
