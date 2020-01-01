use crate::intcode;

pub fn part1() -> std::io::Result<i32> {
    let mut input = Vec::new();
    input.push(1);
    return intcode::execute_with_input("input/d5.txt", (false, 0), &input);
}

pub fn part2() -> std::io::Result<i32> {
    let mut input = Vec::new();
    input.push(5);
    return intcode::execute_with_input("input/d5.txt", (false, 0), &input);
}
