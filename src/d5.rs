use crate::intcode::IntCodeCPU;

pub fn part1() -> std::io::Result<i32> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d5.txt")?;
    cpu.enqueue_input(1);
    cpu.execute();
    return Ok(cpu.last_output.unwrap());
}

pub fn part2() -> std::io::Result<i32> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d5.txt")?;
    cpu.enqueue_input(5);
    cpu.execute();
    return Ok(cpu.last_output.unwrap());
}
