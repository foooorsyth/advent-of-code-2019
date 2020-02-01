use crate::intcode::IntCodeCPU;

pub fn part1() -> std::io::Result<i64> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d19.txt")?;
    let snapshot = cpu.snapshot();
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            cpu.enqueue_input(x as i64);
            cpu.enqueue_input(y as i64);
            cpu.execute();
            count += cpu.last_output.unwrap();
            cpu.load_snapshot(snapshot.snapshot());
        }
    }
    return Ok(count);
}

pub fn part2() -> std::io::Result<i64> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d19.txt")?;
    let snapshot = cpu.snapshot();
    let (x, y) = find(&mut cpu, &snapshot);
    return Ok(x * 10000 + y);
}

fn find(cpu: &mut IntCodeCPU, snapshot: &IntCodeCPU) -> (i64, i64) {
    let mut y = 0;
    loop {
        'x_loop: for x in 0..=y {
            cpu.enqueue_input(x as i64);
            cpu.enqueue_input(y as i64);
            cpu.execute();
            if cpu.last_output.unwrap() == 1 {
                cpu.load_snapshot(snapshot.snapshot());
                cpu.enqueue_input(x + 99);
                cpu.enqueue_input(y);
                cpu.execute();
                if cpu.last_output.unwrap() == 1 {
                    cpu.load_snapshot(snapshot.snapshot());
                    cpu.enqueue_input(x);
                    cpu.enqueue_input(y + 99);
                    cpu.execute();
                    if cpu.last_output.unwrap() == 1 {
                        return (x, y);
                    }
                } else {
                    cpu.load_snapshot(snapshot.snapshot());
                    break 'x_loop;
                }
            }
            cpu.load_snapshot(snapshot.snapshot());
        }
        y += 1;
    }
}
