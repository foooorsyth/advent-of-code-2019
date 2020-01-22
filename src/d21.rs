use crate::intcode::IntCodeCPU;
use crate::shared::ascii_image;

pub fn part1() -> std::io::Result<i64> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d21.txt")?;
    /*
        Program:

        if !A || ((!B || !C) && D) {
            jump
        }
    */
    cpu.execute();
    cpu.enqueue_ascii("NOT B T\n".to_owned());
    cpu.enqueue_ascii("NOT C J\n".to_owned());
    cpu.enqueue_ascii("OR J T\n".to_owned());
    cpu.enqueue_ascii("AND D T\n".to_owned());
    cpu.enqueue_ascii("NOT A J\n".to_owned());
    cpu.enqueue_ascii("OR T J\n".to_owned());
    cpu.enqueue_ascii("WALK\n".to_owned());
    cpu.execute();
    let last_out = cpu.last_output.unwrap();
    if last_out > 127 {
        Ok(last_out)
    } else {
        print!("{}", ascii_image(&cpu.output));
        panic!("Didn't make it across")
    }
}

pub fn part2() -> std::io::Result<i64> {
    let mut cpu = IntCodeCPU::new();
    cpu.read_data_file("input/d21.txt")?;
    /*
        Program:

        if !A || (((!B || !C) && D) && (E || H)) {
            jump
        }
    */
    cpu.execute();
    cpu.enqueue_ascii("OR B T\n".to_owned());
    cpu.enqueue_ascii("AND C T\n".to_owned());
    cpu.enqueue_ascii("NOT T T\n".to_owned());
    cpu.enqueue_ascii("AND D T\n".to_owned());
    cpu.enqueue_ascii("OR E J\n".to_owned());
    cpu.enqueue_ascii("OR H J\n".to_owned());
    cpu.enqueue_ascii("AND J T\n".to_owned());
    cpu.enqueue_ascii("NOT A J\n".to_owned());
    cpu.enqueue_ascii("OR T J\n".to_owned());
    cpu.enqueue_ascii("RUN\n".to_owned());
    cpu.execute();
    let last_out = cpu.last_output.unwrap();
    if last_out > 127 {
        Ok(last_out)
    } else {
        print!("{}", ascii_image(&cpu.output));
        panic!("Didn't make it across")
    }
}
