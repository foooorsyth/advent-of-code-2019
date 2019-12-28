use crate::intcode;

pub fn part1() -> std::io::Result<i32> {
    return intcode::execute("input/d5.txt");
}