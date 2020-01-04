use crate::intcode::IntCodeCPU;

pub fn part1() -> std::io::Result<i64> {
    return common(1);
}

pub fn part2() -> std::io::Result<i64> {
    return common(2);
}

fn common(input: i64) -> std::io::Result<i64> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d9.txt")?;
    cpu.enqueue_input(input);
    cpu.execute();
    return Ok(cpu.last_output.unwrap());
}
