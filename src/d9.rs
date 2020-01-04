use crate::intcode::IntCodeCPU;

pub fn part1() -> std::io::Result<i64> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d9.txt")?;
    cpu.enqueue_input(1);
    cpu.execute();
    return Ok(cpu.last_output.unwrap());
}

pub fn part2() -> std::io::Result<i64> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d9.txt")?;
    cpu.enqueue_input(2);
    cpu.execute();
    return Ok(cpu.last_output.unwrap());
}
