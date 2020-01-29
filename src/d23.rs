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
